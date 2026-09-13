#include <iostream>
#include <chrono>
#include <thread>
#include <iomanip>

struct SensorReading {
    double temperature;
    double humidity;
    long timestamp;
};

double calculateAverageTemperature(const SensorReading* readings, int count) {
    double total = 0.0;

    for (int i = 0; i < count; ++i) {
        total += readings[i].temperature;
    }

    return total / count;
}

int main() {
    const int readingCount = 1'000'000;

    std::cout << "Allocating memory for "
              << readingCount
              << " sensor readings..."
              << std::endl;

    SensorReading* readings = new SensorReading[readingCount];

    for (int i = 0; i < readingCount; ++i) {
        readings[i].temperature = 20.0 + (i % 15) * 0.25;
        readings[i].humidity = 40.0 + (i % 20) * 0.5;
        readings[i].timestamp = 1700000000L + i;
    }

    double average =
        calculateAverageTemperature(readings, readingCount);

    std::cout << std::fixed << std::setprecision(2);
    std::cout << "Average temperature: "
              << average
              << " C"
              << std::endl;

    std::cout << "Memory is currently allocated."
              << std::endl;

    std::this_thread::sleep_for(std::chrono::seconds(8));

    delete[] readings;
    readings = nullptr;

    std::cout << "Memory released manually using delete[]."
              << std::endl;

    std::this_thread::sleep_for(std::chrono::seconds(8));

    return 0;
}
