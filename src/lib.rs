use std::io::ErrorKind;

#[derive(PartialEq, Debug, Clone)]
pub enum Particle {
    Number(i32),
    Unknown(Unknown),
    Symbol(String),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Unknown {
    pub degree: u8,
    pub value: i32,
}

static DEGREES: &[char] = &['¹', '²', '³', '⁴'];

pub fn data_preprocessing(expression: &String) -> Vec<Particle> {
    let mut minus_extend = 1;

    let minus_processing = |n: &str| {
        if n == "-" {
            -1
        } else {
            1
        }
    };

    expression
        .split_whitespace()
        .map(|s| match s.parse::<i32>() {
            Ok(n) => Particle::Number(n * minus_extend),
            Err(_) => match s.chars().position(|x| x == 'x') {
                Some(index) => {
                    if index == s.len() - 1 {
                        return Particle::Unknown(Unknown {
                            degree: 1,
                            value: s[..index]
                                .parse::<i32>()
                                .unwrap_or_else(|_| minus_processing(&s[..index]))
                                * minus_extend,
                        });
                    }

                    if index != s.len() - 3 {
                        panic!(
                            "this expression is incorrect. index: {} / string: {}",
                            index, s
                        );
                    }

                    let degree = s.chars().nth(index + 1).unwrap();

                    let degree = DEGREES
                        .iter()
                        .position(|&c| c == degree)
                        .expect("this expression is incorrect")
                        + 1;

                    return Particle::Unknown(Unknown {
                        degree: degree as u8,
                        value: s[..index]
                            .parse::<i32>()
                            .unwrap_or_else(|_| minus_processing(&s[..index]))
                            * minus_extend,
                    });
                }
                None => {
                    minus_extend = 1;

                    if s == "-" {
                        minus_extend = -1;
                        return Particle::Symbol(String::from("+"));
                    }

                    Particle::Symbol(String::from(s))
                }
            },
        })
        .filter(|p| {
            if let Particle::Number(n) = p {
                if n == &0 {
                    false
                } else {
                    true
                }
            } else {
                true
            }
        })
        .collect()
}

pub fn transposition(particles: &mut Vec<Particle>) {
    let mut equal_index = particles
        .iter()
        .position(|p| p == &Particle::Symbol(String::from("=")))
        .expect("'=' is not included");

    let right_term: Vec<Particle> = particles
        .iter()
        .skip(equal_index + 1)
        .map(|p| match p {
            Particle::Number(n) => Particle::Number(-n),
            Particle::Symbol(sb) => Particle::Symbol(sb.to_string()),
            Particle::Unknown(ukn) => Particle::Unknown(Unknown {
                degree: ukn.degree,
                value: -ukn.value,
            }),
        })
        .collect();

    for (i, p) in right_term.iter().enumerate() {
        particles.remove(equal_index + i + 1);
        particles.insert(equal_index, p.clone());
        equal_index += 1;
    }

    particles.remove(equal_index);
}

pub fn organize_term(particles: Vec<Particle>) -> Vec<Particle> {
    let numbers: Vec<&Particle> = particles
        .iter()
        .filter(|&p| matches!(p, Particle::Number(_n)))
        .collect();

    let mut number = 0;
    for n in numbers.iter() {
        if let Particle::Number(n) = n {
            number += n;
        }
    }

    let unknowns: Vec<&Particle> = particles
        .iter()
        .filter(|&p| matches!(p, Particle::Unknown(_n)))
        .collect();

    let mut highest_degree = 1;
    for ukn in unknowns.iter() {
        if let Particle::Unknown(ukn) = ukn {
            if ukn.degree > highest_degree {
                highest_degree = ukn.degree;
            }
        }
    }

    let mut values: Vec<i32> = Vec::with_capacity(highest_degree as usize);
    for _degree in 0..highest_degree {
        values.push(0);
    }

    for ukn in unknowns.iter() {
        if let Particle::Unknown(ukn) = ukn {
            let value = values.get_mut(ukn.degree as usize - 1).unwrap();
            *value += ukn.value;
        }
    }

    let mut particles: Vec<Particle> = Vec::with_capacity(highest_degree as usize + 1);

    particles.push(Particle::Number(number));

    for (i, value) in values.iter().enumerate() {
        particles.push(Particle::Symbol(String::from("+")));

        particles.push(Particle::Unknown(Unknown {
            degree: (i + 1) as u8,
            value: *value,
        }));
    }

    return particles;
}

pub fn factorization(particles: &Vec<Particle>) -> Result<String, ErrorKind> {
    if particles.len() > 5 {
        panic!("No support for cubic equations or higher");
    }

    if particles.len() == 3 {
        if let Particle::Number(n) = particles.get(0).unwrap() {
            return Ok(format!("x = {}", n * -1));
        }
    }

    if let Particle::Unknown(au) = particles.get(4).expect("a error") {
        let a = au.value;
        if let Particle::Unknown(bu) = particles.get(2).expect("b error") {
            let b = bu.value;
            if let Particle::Number(c) = particles.get(0).expect("c error") {
                let sqrt = f32::sqrt((b.pow(2) - 4 * a * c) as f32);

                let solving = move |squrt: f32| -> String {
                    let result = (-b as f32 + squrt) / (a as f32 * 2.0);

                    if result > 0.0 {
                        return format!("- {}", result);
                    }

                    format!("+ {}", result * -1.0)
                };

                if sqrt == 0.0 {
                    return Ok(format!("(x {})² = 0", solving(0.0)));
                } else {
                    return Ok(format!("(x {})(x {}) = 0", solving(sqrt), solving(-sqrt)));
                }
            }
        }
    }

    Err(ErrorKind::AddrNotAvailable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_preprocessed() {
        let data = data_preprocessing(&String::from("x² + 2x + 1 = 0"));

        let result = vec![
            Particle::Unknown(Unknown {
                degree: 2,
                value: 1,
            }),
            Particle::Symbol(String::from("+")),
            Particle::Unknown(Unknown {
                degree: 1,
                value: 2,
            }),
            Particle::Symbol(String::from("+")),
            Particle::Number(1),
            Particle::Symbol(String::from("=")),
            Particle::Number(0),
        ];

        assert_eq!(data, result);
    }
}
