enum VehicleType {
    Bike,
    Car,
    Truck,
}

trait Vehicle {
    fn get_type(&self) -> String;
}

impl Vehicle for VehicleType {
    fn get_type(&self) -> String {
        match self {
            VehicleType::Bike => "Bike".to_string(),
            VehicleType::Car => "Car".to_string(),
            VehicleType::Truck => "Truck".to_string(),
        }
    }
}

enum VehicleFactory {
    BikeFactory,
    CarFactory,
    TruckFactory,
}

impl VehicleFactory {
    fn create_vehicle(&self) -> Box<dyn Vehicle> {
        match self {
            VehicleFactory::BikeFactory => Box::new(VehicleType::Bike),
            VehicleFactory::CarFactory => Box::new(VehicleType::Car),
            VehicleFactory::TruckFactory => Box::new(VehicleType::Truck),
        }
    }
}

pub fn testing_vehicle_factory() {
    let bike_factory = VehicleFactory::BikeFactory;
    let car_factory = VehicleFactory::CarFactory;
    let truck_factory = VehicleFactory::TruckFactory;

    let bike1 = bike_factory.create_vehicle();
    let bike2 = bike_factory.create_vehicle();
    let car1 = car_factory.create_vehicle();
    let car2 = car_factory.create_vehicle();
    let truck = truck_factory.create_vehicle();

    println!(
        "Here's bike 1's type: {}\n and car 1's type: {}",
        bike1.get_type(),
        car1.get_type()
    );
    println!(
        "\nNow calling for created truck's type: {}\n",
        truck.get_type()
    );
    println!(
        "Finally, here's our second creation of some types we may have already seen from the factories!\n1) {} -- 2) {}",
        bike2.get_type(),
        car2.get_type()
    );
}
