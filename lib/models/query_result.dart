import 'dart:ffi';

import 'package:ffi/ffi.dart';

final class QueryResult extends Struct {
  @Int8()
  external int success;

  @Int32()
  external int affected_rows;

  external Pointer<Utf8> result_ptr;

  @Int32()
  external int result_len;

  external Pointer<Utf8> error_ptr;
}
