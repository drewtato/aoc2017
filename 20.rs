/* Day 20 */

/*
--- Day 20: Particle Swarm ---
Suddenly, the GPU contacts you, asking for help. Someone has asked it to
simulate too many particles, and it won't be able to finish them all in time to
render the next frame at this rate.

It transmits to you a buffer (your puzzle input) listing each particle in order
(starting with particle 0, then particle 1, particle 2, and so on). For each
particle, it provides the X, Y, and Z coordinates for the particle's position
(p), velocity (v), and acceleration (a), each in the format <X,Y,Z>.

Each tick, all particles are updated simultaneously. A particle's properties are
updated in the following order:

Increase the X velocity by the X acceleration.
Increase the Y velocity by the Y acceleration.
Increase the Z velocity by the Z acceleration.
Increase the X position by the X velocity.
Increase the Y position by the Y velocity.
Increase the Z position by the Z velocity.
Because of seemingly tenuous rationale involving z-buffering, the GPU would like
to know which particle will stay closest to position <0,0,0> in the long term.
Measure this using the Manhattan distance, which in this situation is simply the
sum of the absolute values of a particle's X, Y, and Z position.

For example, suppose you are only given two particles, both of which stay
entirely on the X-axis (for simplicity). Drawing the current states of particles
0 and 1 (in that order) with an adjacent a number line and diagram of current X
positions (marked in parenthesis), the following would take place:

p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>                         (0)(1)

p=< 4,0,0>, v=< 1,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
p=< 2,0,0>, v=<-2,0,0>, a=<-2,0,0>                      (1)   (0)

p=< 4,0,0>, v=< 0,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
p=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0>          (1)               (0)

p=< 3,0,0>, v=<-1,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
p=<-8,0,0>, v=<-6,0,0>, a=<-2,0,0>                         (0)
At this point, particle 1 will never be closer to <0,0,0> than particle 0, and
so, in the long run, particle 0 will stay closest.

Which particle will stay closest to position <0,0,0> in the long term?

Your puzzle answer was 157.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---
To simplify the problem further, the GPU would like to remove any particles that
collide. Particles collide if their positions ever exactly match. Because
particles are updated simultaneously, more than two particles can collide at the
same time and place. Once particles collide, they are removed and cannot collide
with anything else after that tick.

For example:

p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>    (0)   (1)   (2)            (3)
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>

p=<-3,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
p=<-1,0,0>, v=< 1,0,0>, a=< 0,0,0>             (0)(1)(2)      (3)
p=< 2,0,0>, v=<-1,0,0>, a=< 0,0,0>

p=< 0,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=< 0,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
p=< 0,0,0>, v=< 1,0,0>, a=< 0,0,0>                       X (3)
p=< 1,0,0>, v=<-1,0,0>, a=< 0,0,0>

------destroyed by collision------
------destroyed by collision------    -6 -5 -4 -3 -2 -1  0  1  2  3
------destroyed by collision------                      (3)
p=< 0,0,0>, v=<-1,0,0>, a=< 0,0,0>
In this example, particles 0, 1, and 2 are simultaneously destroyed at the time
and place marked X. On the next tick, particle 3 passes through unharmed.

How many particles are left after all collisions are resolved?
*/

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("20.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}

	let mut particles = Vec::new();
	for line in input.lines() {
		let mut nums = line.split(|x| (x == '<') || (x == '>') || (x == ','))
								   .filter_map(|x| x.parse::<isize>().ok());
		let mut p = Vec::new();
		for _ in 0..3 {
			let mut v = Vec::new();
			for _ in 0..3 {
				v.push(nums.next().unwrap());
			}
			p.push(v);
		}
		particles.push(p);
	}

	let mut min = 500;
	let mut candidates = Vec::new();
	for (x, p) in particles.iter().enumerate() {
		let acc = p[2].iter().fold(0, |acc, &x| acc + x.abs());
		if acc < min {
			// println!("{}, {}, {:?}", acc, p[1].iter().fold(0, |acc, &x| acc + x.abs()), p);
			min = acc;
			candidates.clear();
		}
		if acc == min {
			candidates.push((x, p))
		}
	}

	let minacc = candidates;
	let mut candidates = Vec::new();
	min = 5000;
	for &(x, p) in minacc.iter() {
		// println!("{:?}", p);
		let acc = p[1].iter().fold(0, |acc, &x| acc + x.abs());
		if acc < min {
			// println!("{}, {}, {:?}", acc, p[1].iter().fold(0, |acc, &x| acc + x.abs()), p);
			min = acc;
			candidates.clear();
		}
		if acc == min {
			candidates.push((x, p))
		}
	}
	if candidates.len() == 1 {
		println!("{}", candidates[0].0);
	} else {
		println!("part 1 ehh");
	}


}