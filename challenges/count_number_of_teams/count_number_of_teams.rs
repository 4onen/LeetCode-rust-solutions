// https://leetcode.com/problems/count-number-of-teams/

pub struct Solution;

// Brute force
// I feel it's too slow for high input length.
// impl Solution {
//     pub fn num_teams(rating: Vec<i32>) -> i32 {
//         assert!(rating.len() >= 3);
//         assert!(rating.len() <= 1000);
//         type Idx = u16;
//         let mut count = 0;
//         for i in 0..rating.len() as Idx {
//             for j in i + 1..rating.len() as Idx {
//                 for k in j + 1..rating.len() as Idx {
//                     if rating[i as usize] < rating[j as usize]
//                         && rating[j as usize] < rating[k as usize]
//                     {
//                         count += 1;
//                     } else if rating[i as usize] > rating[j as usize]
//                         && rating[j as usize] > rating[k as usize]
//                     {
//                         count += 1;
//                     }
//                 }
//             }
//         }
//         count
//     }
// }

// N^2 solution
impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        assert!(rating.len() >= 3);
        assert!(rating.len() <= 1000);
        type Idx = u16;
        type Cnt = i32;
        let mut count = 0;
        for i in 0..rating.len() as Idx {
            let mut left_smaller: Cnt = 0;
            let mut left_larger: Cnt = 0;
            let mut right_smaller: Cnt = 0;
            let mut right_larger: Cnt = 0;
            for j in 0..i {
                if rating[j as usize] < rating[i as usize] {
                    left_smaller += 1;
                } else {
                    left_larger += 1;
                }
            }
            for j in i + 1..rating.len() as Idx {
                if rating[j as usize] < rating[i as usize] {
                    right_smaller += 1;
                } else {
                    right_larger += 1;
                }
            }
            count += left_larger * right_smaller + left_smaller * right_larger;
        }
        count as i32
    }
}

// Reduced-branching version of the above (slower)
// impl Solution {
//     pub fn num_teams(rating: Vec<i32>) -> i32 {
//         assert!(rating.len() >= 3);
//         assert!(rating.len() <= 1000);
//         type Idx = u16;
//         type Cnt = i32;
//         let mut count = 0;
//         for i in 0..rating.len() as Idx {
//             let mut left_smaller = 0;
//             let mut left_larger = 0;
//             let mut right_smaller = 0;
//             let mut right_larger = 0;
//             for j in 0..i {
//                 let cmp = rating[j as usize] < rating[i as usize];
//                 left_smaller += cmp as Cnt;
//                 left_larger += !cmp as Cnt;
//             }
//             for j in i + 1..rating.len() as Idx {
//                 let cmp = rating[j as usize] < rating[i as usize];
//                 right_smaller += cmp as Cnt;
//                 right_larger += !cmp as Cnt;
//             }
//             count += left_smaller * right_larger + left_larger * right_smaller;
//         }
//         count as i32
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(rating: &[i32], expected: i32) {
        assert!(rating.len() >= 3);
        assert!(rating.len() <= 1000);
        let mut seen = std::collections::HashSet::new();
        for &r in rating {
            assert!(r >= 1);
            assert!(r <= 100_000);
            assert!(seen.insert(r));
        }
        assert_eq!(Solution::num_teams(rating.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[2, 5, 3, 4, 1], 3)
    }

    #[test]
    fn ex2() {
        test(&[2, 1, 3], 0)
    }

    #[test]
    fn ex3() {
        test(&[1, 2, 3, 4], 4)
    }

    #[test]
    fn discussion_case1() {
        test(&[2, 3, 1], 0)
    }

    const fn n_choose_3(n: u32) -> u32 {
        match n {
            0 | 1 | 2 => 0,
            3 => 1,
            4.. => n * (n - 1) * (n - 2) / 6,
        }
    }

    #[test]
    fn increasing_ncr() {
        const MAX_INPUT_LEN: i32 = 1000;
        const MAX_INPUT_VAL: i32 = 100_000;
        let input = (1..=MAX_INPUT_LEN)
            .map(|x| x + (MAX_INPUT_VAL - MAX_INPUT_LEN))
            .collect::<Vec<_>>();
        for n in (3..=1000).step_by(100) {
            let expected = n_choose_3(n);
            assert_eq!(
                Solution::num_teams(input[..n as usize].to_vec()),
                expected as i32,
                "n = {}",
                n
            )
        }
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                1000, 1, 999, 2, 998, 3, 997, 4, 996, 5, 995, 6, 994, 7, 993, 8, 992, 9, 991, 10,
                990, 11, 989, 12, 988, 13, 987, 14, 986, 15, 985, 16, 984, 17, 983, 18, 982, 19,
                981, 20, 980, 21, 979, 22, 978, 23, 977, 24, 976, 25,
            ],
            9500,
        );
    }

    #[test]
    fn discussion_case3() {
        test(&[7, 1, 4, 3, 5], 3)
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                1, 101, 201, 301, 401, 501, 601, 701, 801, 901, 1001, 1101, 1201, 1301, 1401, 1501,
                1601, 1701, 1801, 1901, 2001, 2101, 2201, 2301, 2401, 2501, 2601, 2701, 2801, 2901,
                3001, 3101, 3201, 3301, 3401, 3501, 3601, 3701, 3801, 3901, 4001, 4101, 4201, 4301,
                4401, 4501, 4601, 4701, 4801, 4901, 5001, 5101, 5201, 5301, 5401, 5501, 5601, 5701,
                5801, 5901, 6001, 6101, 6201, 6301, 6401, 6501, 6601, 6701, 6801, 6901, 7001, 7101,
                7201, 7301, 7401, 7501, 7601, 7701, 7801, 7901, 8001, 8101, 8201, 8301, 8401, 8501,
                8601, 8701, 8801, 8901, 9001, 9101, 9201, 9301, 9401, 9501, 9601, 9701, 9801, 9901,
                10001, 10101, 10201, 10301, 10401, 10501, 10601, 10701, 10801, 10901, 11001, 11101,
                11201, 11301, 11401, 11501, 11601, 11701, 11801, 11901, 12001, 12101, 12201, 12301,
                12401, 12501, 12601, 12701, 12801, 12901, 13001, 13101, 13201, 13301, 13401, 13501,
                13601, 13701, 13801, 13901, 14001, 14101, 14201, 14301, 14401, 14501, 14601, 14701,
                14801, 14901, 15001, 15101, 15201, 15301, 15401, 15501, 15601, 15701, 15801, 15901,
                16001, 16101, 16201, 16301, 16401, 16501, 16601, 16701, 16801, 16901, 17001, 17101,
                17201, 17301, 17401, 17501, 17601, 17701, 17801, 17901, 18001, 18101, 18201, 18301,
                18401, 18501, 18601, 18701, 18801, 18901, 19001, 19101, 19201, 19301, 19401, 19501,
                19601, 19701, 19801, 19901, 20001, 20101, 20201, 20301, 20401, 20501, 20601, 20701,
                20801, 20901, 21001, 21101, 21201, 21301, 21401, 21501, 21601, 21701, 21801, 21901,
                22001, 22101, 22201, 22301, 22401, 22501, 22601, 22701, 22801, 22901, 23001, 23101,
                23201, 23301, 23401, 23501, 23601, 23701, 23801, 23901, 24001, 24101, 24201, 24301,
                24401, 24501, 24601, 24701, 24801, 24901, 25001, 25101, 25201, 25301, 25401, 25501,
                25601, 25701, 25801, 25901, 26001, 26101, 26201, 26301, 26401, 26501, 26601, 26701,
                26801, 26901, 27001, 27101, 27201, 27301, 27401, 27501, 27601, 27701, 27801, 27901,
                28001, 28101, 28201, 28301, 28401, 28501, 28601, 28701, 28801, 28901, 29001, 29101,
                29201, 29301, 29401, 29501, 29601, 29701, 29801, 29901, 30001, 30101, 30201, 30301,
                30401, 30501, 30601, 30701, 30801, 30901, 31001, 31101, 31201, 31301, 31401, 31501,
                31601, 31701, 31801, 31901, 32001, 32101, 32201, 32301, 32401, 32501, 32601, 32701,
                32801, 32901, 33001, 33101, 33201, 33301, 33401, 33501, 33601, 33701, 33801, 33901,
                34001, 34101, 34201, 34301, 34401, 34501, 34601, 34701, 34801, 34901, 35001, 35101,
                35201, 35301, 35401, 35501, 35601, 35701, 35801, 35901, 36001, 36101, 36201, 36301,
                36401, 36501, 36601, 36701, 36801, 36901, 37001, 37101, 37201, 37301, 37401, 37501,
                37601, 37701, 37801, 37901, 38001, 38101, 38201, 38301, 38401, 38501, 38601, 38701,
                38801, 38901, 39001, 39101, 39201, 39301, 39401, 39501, 39601, 39701, 39801, 39901,
                40001, 40101, 40201, 40301, 40401, 40501, 40601, 40701, 40801, 40901, 41001, 41101,
                41201, 41301, 41401, 41501, 41601, 41701, 41801, 41901, 42001, 42101, 42201, 42301,
                42401, 42501, 42601, 42701, 42801, 42901, 43001, 43101, 43201, 43301, 43401, 43501,
                43601, 43701, 43801, 43901, 44001, 44101, 44201, 44301, 44401, 44501, 44601, 44701,
                44801, 44901, 45001, 45101, 45201, 45301, 45401, 45501, 45601, 45701, 45801, 45901,
                46001, 46101, 46201, 46301, 46401, 46501, 46601, 46701, 46801, 46901, 47001, 47101,
                47201, 47301, 47401, 47501, 47601, 47701, 47801, 47901, 48001, 48101, 48201, 48301,
                48401, 48501, 48601, 48701, 48801, 48901, 49001, 49101, 49201, 49301, 49401, 49501,
                49601, 49701, 49801, 49901, 50001, 50101, 50201, 50301, 50401, 50501, 50601, 50701,
                50801, 50901, 51001, 51101, 51201, 51301, 51401, 51501, 51601, 51701, 51801, 51901,
                52001, 52101, 52201, 52301, 52401, 52501, 52601, 52701, 52801, 52901, 53001, 53101,
                53201, 53301, 53401, 53501, 53601, 53701, 53801, 53901, 54001, 54101, 54201, 54301,
                54401, 54501, 54601, 54701, 54801, 54901, 55001, 55101, 55201, 55301, 55401, 55501,
                55601, 55701, 55801, 55901, 56001, 56101, 56201, 56301, 56401, 56501, 56601, 56701,
                56801, 56901, 57001, 57101, 57201, 57301, 57401, 57501, 57601, 57701, 57801, 57901,
                58001, 58101, 58201, 58301, 58401, 58501, 58601, 58701, 58801, 58901, 59001, 59101,
                59201, 59301, 59401, 59501, 59601, 59701, 59801, 59901, 60001, 60101, 60201, 60301,
                60401, 60501, 60601, 60701, 60801, 60901, 61001, 61101, 61201, 61301, 61401, 61501,
                61601, 61701, 61801, 61901, 62001, 62101, 62201, 62301, 62401, 62501, 62601, 62701,
                62801, 62901, 63001, 63101, 63201, 63301, 63401, 63501, 63601, 63701, 63801, 63901,
                64001, 64101, 64201, 64301, 64401, 64501, 64601, 64701, 64801, 64901, 65001, 65101,
                65201, 65301, 65401, 65501, 65601, 65701, 65801, 65901, 66001, 66101, 66201, 66301,
                66401, 66501, 66601, 66701, 66801, 66901, 67001, 67101, 67201, 67301, 67401, 67501,
                67601, 67701, 67801, 67901, 68001, 68101, 68201, 68301, 68401, 68501, 68601, 68701,
                68801, 68901, 69001, 69101, 69201, 69301, 69401, 69501, 69601, 69701, 69801, 69901,
                70001, 70101, 70201, 70301, 70401, 70501, 70601, 70701, 70801, 70901, 71001, 71101,
                71201, 71301, 71401, 71501, 71601, 71701, 71801, 71901, 72001, 72101, 72201, 72301,
                72401, 72501, 72601, 72701, 72801, 72901, 73001, 73101, 73201, 73301, 73401, 73501,
                73601, 73701, 73801, 73901, 74001, 74101, 74201, 74301, 74401, 74501, 74601, 74701,
                74801, 74901, 75001, 75101, 75201, 75301, 75401, 75501, 75601, 75701, 75801, 75901,
                76001, 76101, 76201, 76301, 76401, 76501, 76601, 76701, 76801, 76901, 77001, 77101,
                77201, 77301, 77401, 77501, 77601, 77701, 77801, 77901, 78001, 78101, 78201, 78301,
                78401, 78501, 78601, 78701, 78801, 78901, 79001, 79101, 79201, 79301, 79401, 79501,
                79601, 79701, 79801, 79901, 80001, 80101, 80201, 80301, 80401, 80501, 80601, 80701,
                80801, 80901, 81001, 81101, 81201, 81301, 81401, 81501, 81601, 81701, 81801, 81901,
                82001, 82101, 82201, 82301, 82401, 82501, 82601, 82701, 82801, 82901, 83001, 83101,
                83201, 83301, 83401, 83501, 83601, 83701, 83801, 83901, 84001, 84101, 84201, 84301,
                84401, 84501, 84601, 84701, 84801, 84901, 85001, 85101, 85201, 85301, 85401, 85501,
                85601, 85701, 85801, 85901, 86001, 86101, 86201, 86301, 86401, 86501, 86601, 86701,
                86801, 86901, 87001, 87101, 87201, 87301, 87401, 87501, 87601, 87701, 87801, 87901,
                88001, 88101, 88201, 88301, 88401, 88501, 88601, 88701, 88801, 88901, 89001, 89101,
                89201, 89301, 89401, 89501, 89601, 89701, 89801, 89901, 90001, 90101, 90201, 90301,
                90401, 90501, 90601, 90701, 90801, 90901, 91001, 91101, 91201, 91301, 91401, 91501,
                91601, 91701, 91801, 91901, 92001, 92101, 92201, 92301, 92401, 92501, 92601, 92701,
                92801, 92901, 93001, 93101, 93201, 93301, 93401, 93501, 93601, 93701, 93801, 93901,
                94001, 94101, 94201, 94301, 94401, 94501, 94601, 94701, 94801, 94901, 95001, 95101,
                95201, 95301, 95401, 95501, 95601, 95701, 95801, 95901, 96001, 96101, 96201, 96301,
                96401, 96501, 96601, 96701, 96801, 96901, 97001, 97101, 97201, 97301, 97401, 97501,
                97601, 97701, 97801, 97901, 98001, 98101, 98201, 98301, 98401, 98501, 98601, 98701,
                98801, 98901, 99001, 99101, 99201, 99301, 99401, 99501, 99601, 99701, 99801, 99901,
            ],
            166167000,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                7, 99992, 8, 99991, 9, 99990, 10, 99989, 11, 99988, 12, 99987, 13, 99986, 14,
                99985, 15, 99984, 16, 99983, 17, 99982, 18, 99981, 19, 99980, 20, 99979, 21, 99978,
                22, 99977, 23, 99976, 24, 99975, 25, 99974, 26, 99973, 27, 99972, 28, 99971, 29,
                99970, 30, 99969, 31, 99968, 32, 99967, 33, 99966, 34, 99965, 35, 99964, 36, 99963,
                37, 99962, 38, 99961, 39, 99960, 40, 99959, 41, 99958, 42, 99957, 43, 99956, 44,
                99955, 45, 99954, 46, 99953, 47, 99952, 48, 99951, 49, 99950, 50, 99949, 51, 99948,
                52, 99947, 53, 99946, 54, 99945, 55, 99944, 56, 99943, 57, 99942, 58, 99941, 59,
                99940, 60, 99939, 61, 99938, 62, 99937, 63, 99936, 64, 99935, 65, 99934, 66, 99933,
                67, 99932, 68, 99931, 69, 99930, 70, 99929, 71, 99928, 72, 99927, 73, 99926, 74,
                99925, 75, 99924, 76, 99923, 77, 99922, 78, 99921, 79, 99920, 80, 99919, 81, 99918,
                82, 99917, 83, 99916, 84, 99915, 85, 99914, 86, 99913, 87, 99912, 88, 99911, 89,
                99910, 90, 99909, 91, 99908, 92, 99907, 93, 99906, 94, 99905, 95, 99904, 96, 99903,
                97, 99902, 98, 99901, 99, 99900, 100, 99899, 101, 99898, 102, 99897, 103, 99896,
                104, 99895, 105, 99894, 106, 99893, 107, 99892, 108, 99891, 109, 99890, 110, 99889,
                111, 99888, 112, 99887, 113, 99886, 114, 99885, 115, 99884, 116, 99883, 117, 99882,
                118, 99881, 119, 99880, 120, 99879, 121, 99878, 122, 99877, 123, 99876, 124, 99875,
                125, 99874, 126, 99873, 127, 99872, 128, 99871, 129, 99870, 130, 99869, 131, 99868,
                132, 99867, 133, 99866, 134, 99865, 135, 99864, 136, 99863, 137, 99862, 138, 99861,
                139, 99860, 140, 99859, 141, 99858, 142, 99857, 143, 99856, 144, 99855, 145, 99854,
                146, 99853, 147, 99852, 148, 99851, 149, 99850, 150, 99849, 151, 99848, 152, 99847,
                153, 99846, 154, 99845, 155, 99844, 156, 99843, 157, 99842, 158, 99841, 159, 99840,
                160, 99839, 161, 99838, 162, 99837, 163, 99836, 164, 99835, 165, 99834, 166, 99833,
                167, 99832, 168, 99831, 169, 99830, 170, 99829, 171, 99828, 172, 99827, 173, 99826,
                174, 99825, 175, 99824, 176, 99823, 177, 99822, 178, 99821, 179, 99820, 180, 99819,
                181, 99818, 182, 99817, 183, 99816, 184, 99815, 185, 99814, 186, 99813, 187, 99812,
                188, 99811, 189, 99810, 190, 99809, 191, 99808, 192, 99807, 193, 99806, 194, 99805,
                195, 99804, 196, 99803, 197, 99802, 198, 99801, 199, 99800, 200, 99799, 201, 99798,
                202, 99797, 203, 99796, 204, 99795, 205, 99794, 206, 99793, 207, 99792, 208, 99791,
                209, 99790, 210, 99789, 211, 99788, 212, 99787, 213, 99786, 214, 99785, 215, 99784,
                216, 99783, 217, 99782, 218, 99781, 219, 99780, 220, 99779, 221, 99778, 222, 99777,
                223, 99776, 224, 99775, 225, 99774, 226, 99773, 227, 99772, 228, 99771, 229, 99770,
                230, 99769, 231, 99768, 232, 99767, 233, 99766, 234, 99765, 235, 99764, 236, 99763,
                237, 99762, 238, 99761, 239, 99760, 240, 99759, 241, 99758, 242, 99757, 243, 99756,
                244, 99755, 245, 99754, 246, 99753, 247, 99752, 248, 99751, 249, 99750, 250, 99749,
                251, 99748, 252, 99747, 253, 99746, 254, 99745, 255, 99744, 256, 99743, 257, 99742,
                258, 99741, 259, 99740, 260, 99739, 261, 99738, 262, 99737, 263, 99736, 264, 99735,
                265, 99734, 266, 99733, 267, 99732, 268, 99731, 269, 99730, 270, 99729, 271, 99728,
                272, 99727, 273, 99726, 274, 99725, 275, 99724, 276, 99723, 277, 99722, 278, 99721,
                279, 99720, 280, 99719, 281, 99718, 282, 99717, 283, 99716, 284, 99715, 285, 99714,
                286, 99713, 287, 99712, 288, 99711, 289, 99710, 290, 99709, 291, 99708, 292, 99707,
                293, 99706, 294, 99705, 295, 99704, 296, 99703, 297, 99702, 298, 99701, 299, 99700,
                300, 99699, 301, 99698, 302, 99697, 303, 99696, 304, 99695, 305, 99694, 306, 99693,
                307, 99692, 308, 99691, 309, 99690, 310, 99689, 311, 99688, 312, 99687, 313, 99686,
                314, 99685, 315, 99684, 316, 99683, 317, 99682, 318, 99681, 319, 99680, 320, 99679,
                321, 99678, 322, 99677, 323, 99676, 324, 99675, 325, 99674, 326, 99673, 327, 99672,
                328, 99671, 329, 99670, 330, 99669, 331, 99668, 332, 99667, 333, 99666, 334, 99665,
                335, 99664, 336, 99663, 337, 99662, 338, 99661, 339, 99660, 340, 99659, 341, 99658,
                342, 99657, 343, 99656, 344, 99655, 345, 99654, 346, 99653, 347, 99652, 348, 99651,
                349, 99650, 350, 99649, 351, 99648, 352, 99647, 353, 99646, 354, 99645, 355, 99644,
                356, 99643, 357, 99642, 358, 99641, 359, 99640, 360, 99639, 361, 99638, 362, 99637,
                363, 99636, 364, 99635, 365, 99634, 366, 99633, 367, 99632, 368, 99631, 369, 99630,
                370, 99629, 371, 99628, 372, 99627, 373, 99626, 374, 99625, 375, 99624, 376, 99623,
                377, 99622, 378, 99621, 379, 99620, 380, 99619, 381, 99618, 382, 99617, 383, 99616,
                384, 99615, 385, 99614, 386, 99613, 387, 99612, 388, 99611, 389, 99610, 390, 99609,
                391, 99608, 392, 99607, 393, 99606, 394, 99605, 395, 99604, 396, 99603, 397, 99602,
                398, 99601, 399, 99600, 400, 99599, 401, 99598, 402, 99597, 403, 99596, 404, 99595,
                405, 99594, 406, 99593, 407, 99592, 408, 99591, 409, 99590, 410, 99589, 411, 99588,
                412, 99587, 413, 99586, 414, 99585, 415, 99584, 416, 99583, 417, 99582, 418, 99581,
                419, 99580, 420, 99579, 421, 99578, 422, 99577, 423, 99576, 424, 99575, 425, 99574,
                426, 99573, 427, 99572, 428, 99571, 429, 99570, 430, 99569, 431, 99568, 432, 99567,
                433, 99566, 434, 99565, 435, 99564, 436, 99563, 437, 99562, 438, 99561, 439, 99560,
                440, 99559, 441, 99558, 442, 99557, 443, 99556, 444, 99555, 445, 99554, 446, 99553,
                447, 99552, 448, 99551, 449, 99550, 450, 99549, 451, 99548, 452, 99547, 453, 99546,
                454, 99545, 455, 99544, 456, 99543, 457, 99542, 458, 99541, 459, 99540, 460, 99539,
                461, 99538, 462, 99537, 463, 99536, 464, 99535, 465, 99534, 466, 99533, 467, 99532,
                468, 99531, 469, 99530, 470, 99529, 471, 99528, 472, 99527, 473, 99526, 474, 99525,
                475, 99524, 476, 99523, 477, 99522, 478, 99521, 479, 99520, 480, 99519, 481, 99518,
                482, 99517, 483, 99516, 484, 99515, 485, 99514, 486, 99513, 487, 99512, 488, 99511,
                489, 99510, 490, 99509, 491, 99508, 492, 99507, 493, 99506, 494, 99505, 495, 99504,
                496, 99503, 497, 99502, 498, 99501, 100000, 99999, 1, 99998, 2, 99997, 3, 99996, 4,
                99995, 5, 99994, 6, 99993, 499,
            ],
            82662032,
        )
    }

    #[test]
    fn discussion_case6() {
        test(&[9, 8, 7, 6, 5, 4], 20)
    }

    #[test]
    fn discussion_case7() {
        test(
            &[
                11, 988, 12, 987, 13, 986, 14, 985, 15, 984, 16, 983, 17, 982, 18, 981, 19, 980,
                20, 979, 21, 978, 22, 977, 23, 976, 24, 975, 25, 974, 26, 973, 27, 972, 28, 971,
                29, 970, 30, 969, 31, 968, 32, 967, 33, 966, 34, 965, 35, 964, 36, 963, 37, 962,
                38, 961, 39, 960, 40, 959, 41, 958, 42, 957, 43, 956, 44, 955, 45, 954, 46, 953,
                47, 952, 48, 951, 49, 950, 50, 949, 51, 948, 52, 947, 53, 946, 54, 945, 55, 944,
                56, 943, 57, 942, 58, 941, 59, 940, 60, 939, 61, 938, 62, 937, 63, 936, 64, 935,
                65, 934, 66, 933, 67, 932, 68, 931, 69, 930, 70, 929, 71, 928, 72, 927, 73, 926,
                74, 925, 75, 924, 76, 923, 77, 922, 78, 921, 79, 920, 80, 919, 81, 918, 82, 917,
                83, 916, 84, 915, 85, 914, 86, 913, 87, 912, 88, 911, 89, 910, 90, 909, 91, 908,
                92, 907, 93, 906, 94, 905, 95, 904, 96, 903, 97, 902, 98, 901, 99, 900, 100, 899,
                101, 898, 102, 897, 103, 896, 104, 895, 105, 894, 106, 893, 107, 892, 108, 891,
                109, 890, 110, 889, 111, 888, 112, 887, 113, 886, 114, 885, 115, 884, 116, 883,
                117, 882, 118, 881, 119, 880, 120, 879, 121, 878, 122, 877, 123, 876, 124, 875,
                125, 874, 126, 873, 127, 872, 128, 871, 129, 870, 130, 869, 131, 868, 132, 867,
                133, 866, 134, 865, 135, 864, 136, 863, 137, 862, 138, 861, 139, 860, 140, 859,
                141, 858, 142, 857, 143, 856, 144, 855, 145, 854, 146, 853, 147, 852, 148, 851,
                149, 850, 150, 849, 151, 848, 152, 847, 153, 846, 154, 845, 155, 844, 156, 843,
                157, 842, 158, 841, 159, 840, 160, 839, 161, 838, 162, 837, 163, 836, 164, 835,
                165, 834, 166, 833, 167, 832, 168, 831, 169, 830, 170, 829, 171, 828, 172, 827,
                173, 826, 174, 825, 175, 824, 176, 823, 177, 822, 178, 821, 179, 820, 180, 819,
                181, 818, 182, 817, 183, 816, 184, 815, 185, 814, 186, 813, 187, 812, 188, 811,
                189, 810, 190, 809, 191, 808, 192, 807, 193, 806, 194, 805, 195, 804, 196, 803,
                197, 802, 198, 801, 199, 800, 200, 799, 201, 798, 202, 797, 203, 796, 204, 795,
                205, 794, 206, 793, 207, 792, 208, 791, 209, 790, 210, 789, 211, 788, 212, 787,
                213, 786, 214, 785, 215, 784, 216, 783, 217, 782, 218, 781, 219, 780, 220, 779,
                221, 778, 222, 777, 223, 776, 224, 775, 225, 774, 226, 773, 227, 772, 228, 771,
                229, 770, 230, 769, 231, 768, 232, 767, 233, 766, 234, 765, 235, 764, 236, 763,
                237, 762, 238, 761, 239, 760, 240, 759, 241, 758, 242, 757, 243, 756, 244, 755,
                245, 754, 246, 753, 247, 752, 248, 751, 249, 750, 250, 749, 251, 748, 252, 747,
                253, 746, 254, 745, 255, 744, 256, 743, 257, 742, 258, 741, 259, 740, 260, 739,
                261, 738, 262, 737, 263, 736, 264, 735, 265, 734, 266, 733, 267, 732, 268, 731,
                269, 730, 270, 729, 271, 728, 272, 727, 273, 726, 274, 725, 275, 724, 276, 723,
                277, 722, 278, 721, 279, 720, 280, 719, 281, 718, 282, 717, 283, 716, 284, 715,
                285, 714, 286, 713, 287, 712, 288, 711, 289, 710, 290, 709, 291, 708, 292, 707,
                293, 706, 294, 705, 295, 704, 296, 703, 297, 702, 298, 701, 299, 700, 300, 699,
                301, 698, 302, 697, 303, 696, 304, 695, 305, 694, 306, 693, 307, 692, 308, 691,
                309, 690, 310, 689, 311, 688, 312, 687, 313, 686, 314, 685, 315, 684, 316, 683,
                317, 682, 318, 681, 319, 680, 320, 679, 321, 678, 322, 677, 323, 676, 324, 675,
                325, 674, 326, 673, 327, 672, 328, 671, 329, 670, 330, 669, 331, 668, 332, 667,
                333, 666, 334, 665, 335, 664, 336, 663, 337, 662, 338, 661, 339, 660, 340, 659,
                341, 658, 342, 657, 343, 656, 344, 655, 345, 654, 346, 653, 347, 652, 348, 651,
                349, 650, 350, 649, 351, 648, 352, 647, 353, 646, 354, 645, 355, 644, 356, 643,
                357, 642, 358, 641, 359, 640, 360, 639, 361, 638, 362, 637, 363, 636, 364, 635,
                365, 634, 366, 633, 367, 632, 368, 631, 369, 630, 370, 629, 371, 628, 372, 627,
                373, 626, 374, 625, 375, 624, 376, 623, 377, 622, 378, 621, 379, 620, 380, 619,
                381, 618, 382, 617, 383, 616, 384, 615, 385, 614, 386, 613, 387, 612, 388, 611,
                389, 610, 390, 609, 391, 608, 392, 607, 393, 606, 394, 605, 395, 604, 396, 603,
                397, 602, 398, 601, 399, 600, 400, 599, 401, 598, 402, 597, 403, 596, 404, 595,
                405, 594, 406, 593, 407, 592, 408, 591, 409, 590, 410, 589, 411, 588, 412, 587,
                413, 586, 414, 585, 415, 584, 416, 583, 417, 582, 418, 581, 419, 580, 420, 579,
                421, 578, 422, 577, 423, 576, 424, 575, 425, 574, 426, 573, 427, 572, 428, 571,
                429, 570, 430, 569, 431, 568, 432, 567, 433, 566, 434, 565, 435, 564, 436, 563,
                437, 562, 438, 561, 439, 560, 440, 559, 441, 558, 442, 557, 443, 556, 444, 555,
                445, 554, 446, 553, 447, 552, 448, 551, 449, 550, 450, 549, 451, 548, 452, 547,
                453, 546, 454, 545, 455, 544, 456, 543, 457, 542, 458, 541, 459, 540, 460, 539,
                461, 538, 462, 537, 463, 536, 464, 535, 465, 534, 466, 533, 467, 532, 468, 531,
                469, 530, 470, 529, 471, 528, 472, 527, 473, 526, 474, 525, 475, 524, 476, 523,
                477, 522, 478, 521, 479, 520, 480, 519, 481, 518, 482, 517, 483, 516, 484, 515,
                485, 514, 486, 513, 487, 512, 488, 511, 489, 510, 490, 509, 491, 508, 492, 507,
                493, 506, 494, 505, 495, 504, 496, 503, 497, 502, 498, 501, 1000, 999, 1, 998, 2,
                997, 3, 996, 4, 995, 5, 994, 6, 993, 7, 992, 8, 991, 9, 990, 10, 989, 499,
            ],
            82592152,
        )
    }
}
