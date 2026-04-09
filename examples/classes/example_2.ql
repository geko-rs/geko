#[ A tractor 🚜 ]#
class Tractor {
    # Init called when a Tractor called
    fn init(self, start_fuel) {
        self.fuel = start_fuel;
    }

    # Does some work
    fn work(self) {
        if self.fuel >= 10.0 {
            self.fuel -= 10.0;
            return "Tractor did some work";
        } else {
            return "Tractor fuel underflow";
        }
    }

    # Fills a fuel
    fn fill(self, fuel) {
        self.fuel += fuel;
    }
}

# Calling tractor
let tractor = Tractor(10.0);
println(tractor.work());
println(tractor.work());
tractor.fill(60.0);
println(tractor.work());
