# Geese (GIS) DBMS

## Introduction

Welcome to Geese GIS DBMS, a Database Management System (DBMS) specifically oriented towards Geographic Information System (GIS) data. This system is implemented in Rust and designed for the management of small GIS datasets stored in the GeoJSON format.

## Features

* **Geospatial Oriented**: Built for the exclusive purpose of handling GIS data.
* **GeoJSON Data Format**: Uses the widely accepted GeoJSON format for encoding a variety of geographic data structures.
* **Lightweight**: Geared towards small datasets, making it a perfect solution for smaller projects.

## Usage Examples
### SELECT
```sql
SELECT * FROM dataset WHERE name = 'ExampleName';
SELECT * FROM dataset ORDER BY ST_Distance(geometry, ST_GeomFromText('POINT(1 2)')) LIMIT 1;
SELECT ST_Distance(a.geometry, b.geometry) FROM dataset a, dataset b WHERE a.id = 123 AND b.id = 456;
```

### UPDATE
```sql
UPDATE dataset SET name = 'NewName' WHERE id = 123;
```

### DELETE
```sql
DELETE FROM dataset WHERE id = 123;
```

### IMPORT
If you manually added a .json file into the data storage, use the IMPORT command to have it recognized by the DBMS:
```sql
IMPORT TABLE 'path/to/your/file.geojson' (id INT, name VARCHAR(100), geometry GEOMETRY) INTO dataset;
```