Day 20's puzzle had a set of connected modules that have different functions. Pulses could be sent through the system - either high or low.

There was a flip-flop module that would alternate the pulse it sends every time it received a low pulse

There was also a conjunction that remembered all previous input pulses, and if they were all high, send a low pulse, otherwise send a high one

Each module could have multiple inputs and outputs

# Part 1
Here, I just simulated the pulses with the provided rules, counting the number of high and low pulses as required

# Part 2
For this, we had to find when a single low pulse was sent to `rx`. This number of pulses was way too large to compute by simulation. My idea was to keep track of any cycles for the conjunctions, so I could skip the simulation, and just use the length of the cycle to chain together each module's cycle length and find the number of button presses to get a single low pulse to `rx`

Instead, I cheated a bit and opted for a simpler solution - I manually found the module that fed into `rx` was a conjunction. So I calculated the cycle lengths for each of the conjunction inputs and used the lowest common multiple
