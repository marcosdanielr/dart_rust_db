import 'dart:ffi';
import 'dart:io';
import 'dart:convert';

import 'package:ffi/ffi.dart';

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

base class Client extends Struct {
  external Pointer<Void> _ptr;
}

class PostgresDriver {
  Pointer<Struct>? _client;

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
    // TODO: deallocate memory in disconnect
  }
}
