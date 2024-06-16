CREATE TABLE ride (
    ride_id VARCHAR PRIMARY KEY,
    rider_id VARCHAR NOT NULL,
    driver_id VARCHAR NOT NULL,
    pickup_location VARCHAR NOT NULL,
    dropoff_location VARCHAR NOT NULL,
    ride_status VARCHAR NOT NULL
)
