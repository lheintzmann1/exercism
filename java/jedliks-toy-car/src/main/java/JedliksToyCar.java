public class JedliksToyCar {
    private int distance;
    private int battery = 100;
    
    public static JedliksToyCar buy() {
        return new JedliksToyCar();
    }

    public String distanceDisplay() {
        return String.format("Driven %d meters", distance);
    }

    public String batteryDisplay() {
        if (battery <= 0) {
            return String.format("Battery empty");
        } else {
            return String.format("Battery at %d%%", battery);
        }
    }

    public void drive() {
        if (battery <= 0) {
            
        } else {
            distance += 20;
            battery -= 1;
        }
    }
}
