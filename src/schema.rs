// @generated automatically by Diesel CLI.

diesel::table! {
    ride (ride_id) {
        ride_id -> Varchar,
        rider_id -> Varchar,
        driver_id -> Varchar,
        pickup_location -> Varchar,
        dropoff_location -> Varchar,
        ride_status -> Varchar,
    }
}
