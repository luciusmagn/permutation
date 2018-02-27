use std::env::args;
use std::process::exit;

#[derive(Debug, Clone)]
struct Program {
	name: char,
	pos: usize,
}

fn main() {
	let mut args = args().skip(1);
	let mut programs: Vec<Program> = if let Some(prgs) = args.next() {
		prgs
			.chars()
			.enumerate()
			.map(|(pos, x)| Program { name: x, pos: pos })
			.collect()
	} else {
		println!("retard");
		exit(-1)
	};

	if let Some(instrukce) = args.next() {
		instrukce
			.split(',')
			.for_each(
				|x| match x.chars().next() {
					Some('s') => (0..x.chars().skip(1).collect::<String>().parse::<usize>().expect("nuh?")).for_each(|_| {let tmp = programs.pop().expect("??"); programs.insert(0, tmp)}),
					Some('x') => programs.swap(
							x.chars().skip(1).collect::<String>().split('/').next().expect("wrong???").parse::<usize>().expect("uuh?"),
							x.chars().skip(1).collect::<String>().split('/').skip(1).next().expect("wrung???").parse::<usize>().expect("huh?"),
					),
					Some('p') => {
						let (i1, i2) = (
							programs
								.iter()
								.enumerate()
								.find(|&(_, e)|
									e.pos ==
									x.chars()
										.skip(1)
										.collect::<String>()
										.split('/')
										.next()
										.expect("wrong???")
										.parse::<usize>()
										.expect("uuh?"))
								.map(|(i, _)| i).expect("no such lelement"),
							programs
								.iter()
								.enumerate()
								.find(|&(_, e)|
									e.pos ==
									x.chars()
										.skip(1)
										.collect::<String>()
										.split('/')
										.skip(1)
										.next()
										.expect("wrung???")
										.parse::<usize>()
										.expect("huh?"))
								.map(|(i, _)| i).expect("no such lelement")
						);
						programs.swap(i1, i2)
					}
					_ => { println!("double fuj"); exit(-2) }
				}
			)
	} else {
		println!("fuj");
		exit(-1)
	}

	println!("result: {}", &programs.iter().map(|x| x.name).collect::<String>())
}
