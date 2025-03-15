import 'dart:ffi';
import 'dart:io';
import 'dart:convert';

import 'package:ffi/ffi.dart';
import 'package:rust_dartbase/models/query_result.dart';

DynamicLibrary _loadPostgresLib() {
  final libNames = {
    'macos': 'target/release/libpostgres.dylib',
    'linux': 'target/release/libpostgres.so',
    'windows': 'target/release/libpostgres.dll',
  };

  final libName = libNames[Platform.operatingSystem];

  if (libName == null) {
    throw Exception('Unsupported platform: ${Platform.operatingSystem}');
  }

  return DynamicLibrary.open(libName);
}

final DynamicLibrary postgresLib = _loadPostgresLib();

final Pointer<Client> Function(Pointer<Int8> connectionUrl)
    connectToPostgresql = postgresLib.lookupFunction<
        Pointer<Client> Function(Pointer<Int8>),
        Pointer<Client> Function(Pointer<Int8>)>('connect_to_postgresql');

final void Function(Pointer<Client>) freePostgresqlConnection =
    postgresLib.lookupFunction<Void Function(Pointer<Client>),
        void Function(Pointer<Client>)>('free_postgresql_connection');

final Pointer<QueryResult> Function(Pointer<Client>, Pointer<Utf8>)
    executeQuery = postgresLib.lookupFunction<
        Pointer<QueryResult> Function(Pointer<Client>, Pointer<Utf8>),
        Pointer<QueryResult> Function(
            Pointer<Client>, Pointer<Utf8>)>('execute_query');

base class Client extends Struct {
  external Pointer<Void> _ptr;
}

class PostgresDriver {
  Pointer<Client>? _client;

  Future<void> connect(String connectionUrl) async {
    final encodedConnectionUrl = Utf8Encoder().convert(connectionUrl);

    final connectionUrlPtr = calloc<Int8>(encodedConnectionUrl.length + 1);
    for (int i = 0; i < encodedConnectionUrl.length; i++) {
      connectionUrlPtr[i] = encodedConnectionUrl[i];
    }
    connectionUrlPtr[encodedConnectionUrl.length] = 0;

    _client = connectToPostgresql(connectionUrlPtr);

    if (_client == nullptr) {
      calloc.free(connectionUrlPtr);
      throw Exception('Failed to connect to PostgreSQL');
    }
  }

  Future<void> disconnect() async {
    if (_client != nullptr) {
      freePostgresqlConnection(_client!);
      _client = nullptr;
    }
  }

  Future<Map<String, dynamic>> executeSqlQuery(String query) async {
    final queryPtr = query.toNativeUtf8();

    try {
      final resultPtr = executeQuery(_client!, queryPtr);

      if (resultPtr != nullptr) {
        final result = resultPtr.ref;

        Map<String, dynamic> resultData = {};

        if (result.success == 1) {
          resultData['success'] = true;
          resultData['affected_rows'] = result.affected_rows;

          if (result.result_ptr != nullptr) {
            final length = result.result_len;

            final resultPointer = result.result_ptr.cast<Uint8>();

            final byteList = resultPointer.asTypedList(length);

            final resultString = String.fromCharCodes(byteList);

            try {
              final jsonResult = jsonDecode(resultString);

              resultData['result'] = jsonResult;
            } catch (e) {
              resultData['result'] = resultString;
            }
          } else {
            print("No result found.");
          }
        } else {
          resultData['success'] = false;

          if (result.error_ptr != nullptr) {
            final errorMessage = result.error_ptr.toDartString();
            resultData['error'] = errorMessage;
            print("Error executing query: $errorMessage");
          } else {
            print(
                "Unknown error: result.success is 0, but no error message available.");
          }
        }

        calloc.free(resultPtr);
        return resultData;
      } else {
        throw Exception('Failed to execute query: result pointer is null.');
      }
    } catch (e) {
      print("Error: $e");
      return {'success': false, 'error': 'Error executing query: $e'};
    } finally {
      calloc.free(queryPtr);
    }
  }
}
