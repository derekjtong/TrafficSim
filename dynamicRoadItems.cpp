#include <iostream>
#include <string>

#include "roadItem.cpp"

class TrafficLight : public DynamicRoadItem {
    bool isGreen;

   public:
    TrafficLight();
    TrafficLight(double x, double y, bool isGreen);
    virtual ~TrafficLight();
    void setGreen();
    void setRed();
    bool isGreen();
};

class Vehicle : public DynamicRoadItem {
    std::string model;
    double speed;
    double direction;

   public:
    Vehicle();
    Vehicle(double speed, double location, double direction, std::string model);
    virtual ~Vehicle();
    std::string getModel();
    std::string setModel();
    virtual void accelerate(double toSpeed);
    virtual void decelerate(double toSpeed);
    void turn(double direction, double degrees);
    double getSpeed();
    double getLocation();
    double getDirection();
};

class Truck : public Vehicle {
    double loadWeight;

   public:
    Truck();
    Truck(double speed, double location, double direction, double loadWeight);
    ~Truck() override;
    // override; truck will have a slower acceleration/deceleration
    void accelerate(double toSpeed) override;
    void decelerate(double toSpeed) override;
    void setLoadWeight(double loadWeight);
    double getCargoWeight();
};

class Car : public Vehicle {
   public:
    Car();
    Car(double speed, double location, double direction);
    ~Car() override;
};