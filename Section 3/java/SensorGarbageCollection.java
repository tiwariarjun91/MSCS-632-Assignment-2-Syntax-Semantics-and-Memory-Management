public class SensorGarbageCollection {
    static class SensorReading {
        double temperature;
        double humidity;

        SensorReading(double temperature, double humidity) {
            this.temperature = temperature;
            this.humidity = humidity;
        }
    }

    public static void main(String[] args) throws InterruptedException {
        int numberOfReadings = 1_000_000;
        System.out.println("Creating sensor readings...");
        SensorReading[] readings = new SensorReading[numberOfReadings];

        for (int i = 0; i < numberOfReadings; i++) {
            readings[i] = new SensorReading(
                    20.0 + (i % 10),
                    40.0 + (i % 20)
            );
        }

        System.out.println("Sensor readings are stored in memory.");
        Thread.sleep(30000);
        readings = null;

        System.out.println("Reference to sensor readings removed.");
        System.out.println("The objects are now eligible for garbage collection.");

        System.gc();
        Thread.sleep(30000);
        System.out.println("Program finished.");
    }
}