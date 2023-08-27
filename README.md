# Restraint

This is an experiment to see what happens if your 2D constraint engine allows for "soft" constraints rather than strictly "hard" constraints.

To constraint the distance between two points we add a spring between them with a rest length equal to the desired distance. To constrain the angle of a line formed by two points we add a torsion spring with a rest angle equal to the desired angle.

Points are modeled as point masses, and all springs have some damping applied.

## Example
```
let mut system = System::new();

// Add three points which happen to form a right triangle
let pt_a_id = system.add_point(0.0, 0.0);
let pt_b_id = system.add_point(1.0, 1.0);
let pt_c_id = system.add_point(1.0, 0.0);

// constrain the lengths of each line segment
let spring_a_id = system.add_spring(pt_b_id, pt_c_id, 1.0);
let spring_b_id = system.add_spring(pt_a_id, pt_c_id, 1.0);
let spring_c_id = system.add_spring(pt_a_id, pt_b_id, 1.0);

// constrain the AB line segment to be vertical
let spring_d_id = system.add_torsion(pt_a_id, pt_b_id, PI / 2.0);

system.solve(100);
```

The solver is just computing all the forces on all the points, them stepping them forward some small dt in a loop. After each loop the solver emits the full state vector to stdout. It looks like:

```
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `/Users/matthewferraro/Documents/git/restraint/target/debug/restraint`
0,0,0,0,0,0,1,1,0,0,0,0,1,0,0,0,0,0
0.06921749262439306,-0.021417319314037606,1.6965071721664966,-0.5249342969126864,1.6965071721664966,-0.5249342969126864,0.930782507375607,1.0214173193140377,-1.6965071721664966,0.5249342969126864,-1.6965071721664966,0.5249342969126864,1,0,0,0,0,0
0.13893716808942655,-0.05035663462096522,1.7420804216364385,-0.7195897731484746,0.04557324946994196,-0.1946554762357882,0.8354892128551406,1.0400220653478702,-2.368884810249992,0.46629150665104585,-0.6723776380834953,-0.0586427902616406,1.025573619055433,0.010334569273095097,0.6268043886135534,0.2532982664974288,0.6268043886135534,0.2532982664974288
...
```

To visualize how the system changes over time I've also added a python script that uses matplotlib to generate an animation. To set that up:

```
python3 -m venv venv
. venv/bin/activate
pip install -r requirements.txt
```

Then run it all together like:

```
cargo run > results.txt && python plot.py && open movie.mp4
```

[movie goes here]

## Notes

Yes, everything lives in `main.rs` right now. The older versions of main with suffixes like `main_v1.rs` just represent earlier attempts that I abandoned.