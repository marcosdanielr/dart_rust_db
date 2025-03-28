import 'package:rust_dartbase/src/drivers/postgres_driver.dart';
import 'package:test/test.dart';
import 'dart:io';

void main() {
  group('PostgresDriver', () {
    test('connect to PostgreSQL successfully', () async {
      final driver = PostgresDriver();
      final connectionString = Platform.environment['TEST_DATABASE_URL'];

      if (connectionString == null) {
        throw Exception('TEST_DATABASE_URL environment variable not set.');
      }

      expect(driver.connect(connectionString), completes);
    });

    test('connect to PostgreSQL failed', () async {
      final driver = PostgresDriver();
      final connectionString =
          'postgresql://user:password@host:port/nonexistent_database';

      expect(driver.connect(connectionString), throwsA(isA<Exception>()));
    });
  });
}
