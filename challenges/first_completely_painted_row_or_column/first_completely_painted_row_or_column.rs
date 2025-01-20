// https://leetcode.com/problems/first-completely-painted-row-or-column/

pub struct Solution;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        type Idx = u32;
        type Cnt = u32;
        let n = mat.len() as Idx;
        let m = mat[0].len() as Idx;
        let mut row_painted = vec![0 as Cnt; n as usize];
        let mut col_painted = vec![0 as Cnt; m as usize];
        let mut el_map = vec![(Idx::MAX, Idx::MAX); arr.len() + 1];
        for i in 0..n {
            for j in 0..m {
                let el = mat[i as usize][j as usize];
                el_map[el as usize] = (i, j);
            }
        }
        for (i, el) in arr.into_iter().enumerate() {
            let (r, c) = el_map[el as usize];
            row_painted[r as usize] += 1;
            if row_painted[r as usize] >= m {
                return i as i32;
            }
            col_painted[c as usize] += 1;
            if col_painted[c as usize] >= n {
                return i as i32;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(arr: &[i32], mat: &[&[i32]], expected: i32) {
        assert!(mat.len() >= 1);
        assert!(mat.len() <= 100_000);
        assert!(mat[0].len() >= 1);
        assert!(mat[0].len() <= 100_000);
        let els = mat.len() * mat[0].len();
        assert!(els <= 100_000);
        let mut seen = vec![false; els + 1];
        for &row in mat {
            assert_eq!(row.len(), mat[0].len());
            for &el in row {
                assert!(el >= 1);
                assert!(el <= els as i32);
                assert!(!seen[el as usize]);
                seen[el as usize] = true;
            }
        }
        assert_eq!(arr.len(), mat.len() * mat[0].len());
        let mut seen = vec![false; els + 1];
        for &el in arr {
            assert!(el >= 1);
            assert!(el <= els as i32);
            assert!(!seen[el as usize]);
            seen[el as usize] = true;
        }
        assert_eq!(
            Solution::first_complete_index(arr.to_vec(), mat.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 3, 2, 4], &[&[1, 4], &[2, 3]], 2)
    }

    #[test]
    fn ex2() {
        test(
            &[2, 8, 7, 4, 1, 3, 5, 6, 9],
            &[&[3, 2, 5], &[1, 4, 6], &[8, 7, 9]],
            3,
        )
    }

    #[test]
    fn discussion_case1() {
        test(&[6, 2, 3, 1, 4, 5], &[&[5, 1], &[2, 4], &[6, 3]], 2)
    }

    #[test]
    fn discussion_case1_1() {
        test(&[2, 3, 1, 4], &[&[3, 1], &[2, 4]], 1);
        test(&[4, 3, 1, 2], &[&[3, 1], &[2, 4]], 2);
    }

    #[test]
    fn discussion_case2() {
        test(
            &[8, 2, 3, 5, 6, 7, 4, 1, 9, 10],
            &[&[6, 9], &[7, 4], &[1, 10], &[3, 8], &[2, 5]],
            2,
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            &[
                880, 112, 944, 845, 878, 357, 835, 200, 599, 276, 755, 908, 171, 465, 434, 124,
                574, 159, 575, 1037, 428, 44, 401, 813, 391, 810, 355, 970, 387, 802, 164, 75, 638,
                218, 702, 849, 609, 192, 435, 500, 742, 211, 985, 10, 65, 232, 310, 376, 815, 385,
                989, 704, 255, 630, 602, 416, 943, 63, 380, 826, 864, 415, 235, 93, 138, 150, 513,
                35, 131, 531, 283, 268, 432, 934, 165, 552, 827, 167, 862, 298, 724, 730, 146, 577,
                583, 334, 787, 665, 1048, 358, 339, 69, 109, 713, 527, 121, 562, 445, 570, 576,
                734, 597, 495, 953, 978, 50, 322, 890, 252, 694, 388, 452, 778, 752, 671, 760, 875,
                17, 829, 172, 94, 356, 254, 758, 317, 629, 859, 764, 785, 173, 1041, 338, 912,
                1042, 591, 632, 993, 503, 424, 940, 221, 58, 965, 961, 488, 1029, 361, 619, 280,
                274, 612, 511, 353, 968, 636, 207, 680, 937, 516, 850, 870, 202, 544, 67, 550, 741,
                902, 473, 214, 343, 928, 918, 995, 99, 814, 700, 318, 312, 533, 735, 588, 884, 959,
                557, 394, 483, 753, 36, 991, 1018, 161, 366, 938, 521, 822, 364, 378, 942, 932,
                560, 239, 332, 1019, 315, 449, 1021, 505, 479, 539, 746, 966, 801, 285, 856, 684,
                756, 38, 421, 851, 789, 289, 253, 101, 163, 523, 156, 373, 949, 340, 384, 846, 986,
                568, 898, 277, 198, 554, 45, 881, 166, 196, 72, 368, 980, 408, 598, 830, 541, 248,
                128, 407, 13, 982, 264, 49, 406, 586, 6, 477, 133, 352, 593, 718, 858, 871, 333,
                87, 573, 721, 243, 716, 698, 168, 74, 687, 653, 78, 900, 524, 233, 230, 738, 962,
                16, 92, 903, 486, 153, 1011, 526, 82, 395, 1, 379, 579, 100, 157, 906, 1027, 89,
                314, 578, 447, 740, 767, 535, 371, 1002, 508, 542, 344, 51, 476, 145, 872, 745,
                794, 939, 920, 260, 692, 485, 455, 349, 345, 548, 967, 907, 610, 76, 545, 359, 998,
                68, 1017, 945, 410, 213, 981, 892, 726, 466, 134, 493, 566, 188, 628, 129, 556,
                728, 220, 257, 765, 389, 431, 873, 720, 639, 236, 712, 225, 987, 853, 958, 326,
                916, 175, 960, 587, 386, 116, 130, 60, 98, 147, 688, 883, 1016, 184, 422, 1034,
                420, 748, 203, 90, 404, 481, 217, 1040, 1014, 832, 843, 23, 250, 1024, 924, 433,
                115, 865, 1010, 15, 271, 777, 1004, 582, 922, 300, 205, 242, 811, 346, 595, 52, 39,
                1054, 831, 296, 820, 249, 278, 4, 905, 661, 861, 1012, 1005, 644, 231, 631, 467,
                855, 799, 617, 14, 238, 25, 840, 114, 637, 494, 515, 482, 286, 751, 284, 34, 382,
                532, 608, 565, 614, 187, 80, 446, 70, 749, 874, 11, 136, 784, 954, 127, 311, 714,
                103, 215, 97, 641, 295, 901, 372, 362, 176, 228, 381, 273, 444, 528, 514, 186, 403,
                988, 759, 839, 1053, 522, 984, 805, 377, 976, 185, 178, 863, 227, 154, 413, 695,
                657, 365, 1009, 739, 701, 774, 868, 669, 519, 963, 469, 600, 803, 893, 88, 772,
                297, 1022, 769, 62, 564, 654, 780, 804, 913, 335, 992, 869, 189, 651, 947, 842,
                866, 383, 703, 29, 1050, 921, 443, 994, 699, 951, 946, 263, 411, 1051, 487, 402,
                48, 398, 696, 222, 923, 85, 615, 655, 707, 3, 191, 971, 616, 622, 41, 490, 265,
                625, 83, 454, 507, 779, 722, 941, 325, 841, 525, 997, 931, 216, 549, 491, 607, 793,
                618, 468, 731, 606, 251, 309, 496, 567, 499, 652, 275, 666, 571, 351, 1003, 158,
                392, 925, 104, 635, 534, 624, 990, 425, 1056, 464, 563, 396, 969, 690, 709, 888,
                393, 529, 399, 973, 27, 647, 952, 510, 930, 999, 370, 743, 102, 658, 604, 828, 195,
                1013, 1020, 347, 879, 105, 155, 126, 589, 506, 817, 427, 237, 436, 21, 623, 1000,
                330, 725, 627, 782, 885, 927, 596, 451, 1007, 439, 732, 348, 773, 256, 144, 204,
                54, 512, 120, 910, 199, 201, 160, 603, 12, 540, 33, 409, 57, 833, 857, 538, 390,
                118, 478, 43, 110, 659, 697, 1025, 673, 1039, 497, 786, 79, 244, 886, 305, 328,
                825, 152, 308, 1038, 594, 504, 2, 412, 791, 471, 983, 685, 561, 1045, 964, 95, 509,
                678, 306, 32, 649, 517, 122, 262, 660, 955, 91, 37, 771, 9, 73, 26, 797, 71, 737,
                559, 558, 375, 518, 800, 783, 894, 86, 245, 1055, 108, 663, 489, 790, 324, 781,
                137, 64, 30, 316, 1043, 292, 676, 882, 761, 350, 717, 634, 686, 1049, 727, 837,
                414, 84, 132, 664, 640, 642, 208, 689, 919, 580, 823, 405, 555, 113, 551, 889, 662,
                1006, 323, 125, 321, 867, 293, 1031, 226, 24, 895, 429, 423, 806, 936, 170, 776,
                972, 844, 711, 400, 442, 956, 775, 81, 119, 498, 259, 788, 770, 1008, 291, 459, 19,
                290, 584, 22, 140, 472, 621, 601, 502, 914, 313, 1023, 1030, 747, 492, 656, 838,
                460, 808, 61, 909, 437, 887, 417, 440, 287, 66, 691, 729, 197, 463, 77, 1044, 948,
                648, 419, 320, 682, 750, 929, 397, 816, 8, 279, 674, 896, 143, 877, 650, 270, 854,
                418, 329, 180, 219, 426, 240, 269, 369, 536, 224, 234, 911, 374, 824, 611, 319,
                672, 795, 28, 744, 979, 210, 246, 1026, 675, 796, 307, 818, 241, 708, 281, 474,
                643, 677, 1028, 754, 1033, 821, 282, 53, 7, 933, 792, 169, 547, 457, 190, 111, 546,
                59, 530, 206, 590, 135, 798, 342, 341, 181, 299, 706, 470, 367, 667, 950, 766, 977,
                461, 106, 151, 736, 480, 123, 626, 763, 179, 620, 229, 462, 592, 162, 705, 834,
                272, 974, 847, 430, 142, 301, 646, 683, 107, 360, 679, 935, 363, 304, 807, 453,
                733, 926, 139, 182, 719, 1015, 852, 331, 605, 141, 149, 302, 148, 572, 520, 209,
                904, 458, 1036, 581, 354, 613, 31, 247, 303, 47, 633, 266, 812, 194, 337, 288, 768,
                710, 223, 294, 645, 681, 891, 55, 915, 441, 484, 42, 553, 899, 475, 975, 261, 819,
                543, 193, 1052, 336, 1035, 860, 96, 693, 438, 670, 715, 501, 809, 267, 174, 1047,
                20, 957, 46, 836, 848, 456, 723, 668, 117, 448, 1046, 1001, 40, 18, 762, 450, 212,
                585, 5, 1032, 258, 569, 757, 183, 917, 537, 56, 876, 996, 177, 897, 327,
            ],
            &[
                &[383, 344, 152, 610, 1024, 633, 561, 238, 409, 649, 219],
                &[128, 91, 234, 94, 531, 60, 813, 6, 110, 326, 396],
                &[701, 983, 29, 768, 233, 739, 616, 487, 956, 255, 135],
                &[282, 516, 733, 637, 919, 1005, 941, 38, 217, 572, 251],
                &[735, 850, 456, 488, 42, 932, 345, 597, 830, 175, 99],
                &[695, 441, 459, 477, 145, 332, 411, 952, 989, 241, 600],
                &[549, 1025, 168, 680, 165, 471, 822, 843, 762, 596, 953],
                &[721, 878, 1003, 874, 577, 237, 964, 652, 360, 218, 285],
                &[1015, 296, 759, 212, 78, 1018, 955, 664, 209, 181, 720],
                &[137, 651, 176, 914, 644, 183, 186, 563, 40, 347, 860],
                &[243, 694, 4, 496, 986, 973, 63, 511, 357, 157, 761],
                &[692, 377, 765, 668, 578, 73, 121, 766, 931, 944, 772],
                &[999, 814, 891, 191, 44, 400, 484, 1012, 791, 192, 1017],
                &[3, 143, 537, 950, 362, 236, 378, 795, 1037, 359, 75],
                &[707, 138, 1009, 124, 785, 690, 1031, 228, 1020, 783, 1035],
                &[789, 938, 302, 242, 906, 61, 593, 832, 689, 917, 414],
                &[444, 55, 828, 589, 598, 535, 780, 734, 452, 703, 298],
                &[773, 367, 17, 361, 924, 574, 907, 8, 615, 434, 428],
                &[534, 671, 576, 190, 342, 717, 153, 797, 527, 481, 161],
                &[115, 327, 259, 916, 1021, 80, 66, 525, 849, 653, 712],
                &[287, 148, 872, 996, 37, 381, 908, 752, 254, 1019, 447],
                &[335, 825, 881, 64, 79, 54, 261, 659, 1047, 551, 264],
                &[306, 1014, 415, 387, 83, 852, 464, 636, 566, 681, 201],
                &[392, 711, 806, 423, 200, 585, 69, 1001, 876, 491, 629],
                &[1036, 299, 682, 398, 882, 50, 350, 892, 141, 565, 108],
                &[575, 1026, 646, 329, 543, 570, 43, 433, 376, 252, 546],
                &[22, 861, 604, 156, 163, 166, 24, 45, 968, 160, 778],
                &[1056, 845, 389, 109, 557, 58, 399, 847, 788, 541, 277],
                &[379, 250, 158, 380, 458, 508, 753, 184, 278, 853, 248],
                &[232, 980, 995, 764, 28, 92, 178, 358, 802, 736, 700],
                &[349, 526, 366, 95, 708, 316, 923, 197, 408, 627, 129],
                &[312, 144, 394, 328, 545, 697, 889, 826, 550, 174, 787],
                &[469, 106, 552, 149, 757, 977, 809, 451, 688, 123, 224],
                &[313, 1039, 101, 87, 530, 417, 665, 623, 954, 848, 831],
                &[100, 14, 580, 509, 7, 718, 374, 837, 501, 495, 230],
                &[820, 507, 517, 427, 203, 293, 581, 710, 899, 325, 1000],
                &[159, 384, 263, 929, 647, 81, 823, 56, 267, 965, 536],
                &[503, 457, 540, 660, 371, 817, 638, 634, 867, 440, 693],
                &[1027, 480, 220, 639, 532, 476, 1046, 767, 403, 105, 478],
                &[416, 330, 927, 657, 404, 781, 746, 82, 221, 98, 790],
                &[625, 658, 640, 608, 195, 724, 364, 573, 945, 728, 271],
                &[913, 401, 146, 656, 346, 1041, 603, 1030, 1040, 284, 887],
                &[150, 676, 769, 365, 483, 1028, 910, 119, 684, 280, 422],
                &[793, 829, 454, 691, 35, 742, 893, 107, 229, 53, 672],
                &[981, 567, 372, 71, 62, 331, 756, 151, 779, 643, 320],
                &[1050, 19, 111, 405, 421, 397, 385, 771, 522, 269, 86],
                &[946, 678, 258, 410, 935, 1011, 16, 645, 859, 547, 492],
                &[994, 520, 23, 542, 1007, 716, 425, 187, 1, 47, 1053],
                &[601, 833, 210, 523, 666, 226, 279, 300, 846, 286, 856],
                &[231, 198, 206, 571, 518, 920, 354, 46, 528, 871, 675],
                &[51, 727, 339, 5, 928, 30, 420, 544, 67, 978, 648],
                &[353, 863, 274, 951, 613, 205, 472, 352, 502, 602, 810],
                &[553, 319, 65, 164, 247, 498, 322, 1004, 958, 72, 590],
                &[895, 76, 1032, 120, 303, 864, 310, 588, 85, 57, 133],
                &[635, 294, 289, 883, 136, 868, 402, 982, 317, 1044, 196],
                &[696, 904, 134, 323, 333, 391, 273, 939, 963, 130, 698],
                &[140, 479, 240, 142, 1033, 663, 270, 96, 834, 426, 249],
                &[770, 239, 731, 1013, 446, 1023, 188, 276, 560, 470, 283],
                &[754, 947, 599, 10, 796, 991, 52, 824, 315, 900, 351],
                &[606, 260, 494, 429, 719, 704, 216, 179, 595, 461, 654],
                &[957, 324, 513, 870, 866, 619, 948, 840, 268, 940, 412],
                &[1052, 295, 122, 340, 485, 189, 836, 915, 311, 609, 390],
                &[884, 368, 827, 253, 564, 370, 936, 614, 558, 512, 443],
                &[256, 467, 763, 1055, 32, 395, 642, 48, 903, 699, 902],
                &[975, 838, 808, 214, 539, 90, 709, 348, 722, 811, 841],
                &[921, 116, 489, 514, 750, 373, 748, 49, 612, 877, 131],
                &[305, 949, 199, 465, 12, 338, 343, 337, 59, 204, 816],
                &[213, 1010, 375, 1029, 321, 382, 448, 20, 21, 127, 9],
                &[15, 556, 880, 281, 751, 460, 901, 686, 937, 803, 926],
                &[41, 182, 729, 112, 1002, 961, 784, 726, 911, 275, 933],
                &[39, 529, 26, 318, 782, 473, 875, 674, 125, 679, 27],
                &[1022, 474, 855, 667, 744, 341, 113, 835, 235, 650, 84],
                &[894, 821, 943, 611, 974, 177, 579, 225, 562, 702, 801],
                &[879, 755, 304, 103, 424, 621, 36, 655, 661, 799, 207],
                &[1006, 760, 291, 272, 987, 170, 942, 687, 631, 685, 314],
                &[705, 155, 13, 723, 673, 776, 486, 777, 632, 499, 308],
                &[706, 873, 493, 538, 909, 858, 617, 167, 215, 11, 68],
                &[185, 257, 288, 1043, 524, 369, 715, 211, 738, 406, 430],
                &[886, 641, 885, 607, 990, 594, 126, 586, 1051, 905, 985],
                &[972, 591, 462, 890, 869, 356, 519, 386, 31, 925, 500],
                &[431, 227, 435, 582, 966, 807, 1048, 842, 173, 506, 997],
                &[388, 139, 554, 912, 439, 1049, 979, 438, 70, 246, 862],
                &[818, 2, 737, 669, 758, 515, 455, 208, 169, 967, 419],
                &[102, 630, 819, 959, 775, 714, 418, 442, 1008, 497, 510],
                &[992, 922, 1038, 475, 1045, 605, 118, 618, 774, 971, 147],
                &[437, 740, 193, 290, 622, 463, 1016, 292, 505, 222, 857],
                &[745, 815, 713, 592, 117, 74, 89, 1034, 844, 898, 104],
                &[583, 865, 792, 407, 670, 307, 449, 171, 934, 569, 662],
                &[747, 336, 194, 626, 245, 741, 266, 202, 334, 466, 897],
                &[77, 309, 976, 854, 743, 620, 730, 363, 132, 88, 244],
                &[34, 432, 468, 800, 93, 888, 798, 453, 172, 624, 114],
                &[998, 33, 555, 1054, 918, 265, 521, 786, 677, 262, 962],
                &[301, 180, 683, 851, 25, 970, 732, 839, 533, 482, 504],
                &[490, 154, 794, 930, 297, 805, 988, 812, 450, 969, 18],
                &[162, 97, 413, 896, 584, 223, 559, 993, 628, 355, 568],
                &[445, 960, 587, 725, 548, 749, 1042, 984, 804, 436, 393],
            ],
            645,
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            &[2, 10, 8, 3, 12, 6, 5, 1, 11, 7, 9, 4],
            &[&[2, 9, 6, 8], &[7, 5, 10, 3], &[4, 1, 11, 12]],
            4,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            &[10, 12, 1, 7, 2, 6, 9, 11, 8, 5, 3, 4],
            &[&[8, 1, 6, 10], &[5, 9, 2, 4], &[12, 3, 7, 11]],
            5,
        )
    }
}
