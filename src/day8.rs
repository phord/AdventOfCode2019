#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ TESTS
#[test] fn test_day8_part1() { assert_eq!(solve("123456789012", (3,2), 1), 1); }


//------------------------------ SOLVE

fn solve(input: &'static str, size: (usize, usize), _part: usize) -> usize {
    let (w, h) = size;
    let mut min = h * w;
    let mut ans = 0;
    let mut image = vec![2; h*w];
    for layer in input.as_bytes().chunks(h * w) {
        if layer.len() == h*w {
            // dbg!(std::str::from_utf8(layer).unwrap());

            // Build image by stacking layers
            for (i, b) in layer.iter().enumerate() {
                if *b < b'2' && image[i] == 2 {
                    image[i] = *b - b'0';
                }
            }

            let zeros = layer.iter().filter(|&&x| x==b'0').count();
            if zeros < min {
                min = zeros;
                let ones = layer.iter().filter(|&&x| x==b'1').count();
                let twos = layer.iter().filter(|&&x| x==b'2').count();
                ans = ones * twos;
                // dbg!(zeros, ones, twos, min, ans);
            }
        }
    }

    for row in 0..h {
        let xx = image[row*w..row*w+w].iter().map(|&x| if x == 0 {" "} else {"#"}).collect::<String>();
        println!("{}", xx);
    }
    ans
}

fn solve1(input: &'static str) -> usize { solve(input, (25,6), 1) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day8, part1)]
fn day8_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    // assert_eq!(ans, 0);
    ans
}


//------------------------------ SAMPLE DATA
const _SAMPLE: &str = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
const _CODE: &str = "#  #  ##  #  # ####  ##
# #  #  # #  #    # #  #
##   #  # #  #   #  #  #
# #  #### #  #  #   ####
# #  #  # #  # #    #  #
#  # #  #  ##  #### #  # ";