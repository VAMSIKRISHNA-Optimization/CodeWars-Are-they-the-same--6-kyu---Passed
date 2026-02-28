# CodeWars-Are-they-the-same--6-kyu---Passed
Given two arrays a and b write a function comp(a, b) (orcompSame(a, b)) that checks whether the two arrays have the "same" elements, with the same multiplicities (the multiplicity of a member is the number of times it appears). "Same" means, here, that the elements in b are the elements in a squared, regardless of the order.

Examples
Valid arrays
a = [121, 144, 19, 161, 19, 144, 19, 11]  
b = [121, 14641, 20736, 361, 25921, 361, 20736, 361]
comp(a, b) returns true because in b 121 is the square of 11, 14641 is the square of 121, 20736 the square of 144, 361 the square of 19, 25921 the square of 161, and so on. It gets obvious if we write b's elements in terms of squares:

a = [121, 144, 19, 161, 19, 144, 19, 11] 
b = [11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19]
Invalid arrays
If, for example, we change the first number to something else, comp is not returning true anymore:

a = [121, 144, 19, 161, 19, 144, 19, 11]  
b = [132, 14641, 20736, 361, 25921, 361, 20736, 361]
comp(a,b) returns false because in b 132 is not the square of any number of a.

a = [121, 144, 19, 161, 19, 144, 19, 11]  
b = [121, 14641, 20736, 36100, 25921, 361, 20736, 361]
comp(a,b) returns false because in b 36100 is not the square of any number of a.

Remarks
a or b might be [] or {} (all languages except R, Shell).
a or b might be nil or null or None or nothing (except in C++, COBOL, Crystal, D, Dart, Elixir, Fortran, F#, Haskell, Nim, OCaml, Pascal, Perl, PowerShell, Prolog, PureScript, R, Racket, Rust, Shell, Swift).
If a or b are nil (or null or None, depending on the language), the problem doesn't make sense so return false.

// TEST CASES
fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
    assert_eq!(comp(a, b), exp)
}

#[test]
fn tests_comp() {

    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, false);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*11, 121*121, 144*144, 190*190, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, false);
    let a1 = vec![];
    let a2 = vec![];
    testing(a1, a2, true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11, 1008];
    let a2 = vec![11*11, 121*121, 144*144, 190*190, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, false);
    let a1 = vec![10000000, 100000000];
    let a2 = vec![10000000 * 10000000, 100000000 * 100000000];
    testing(a1, a2, true);
    let a1 = vec![10000001, 100000000];
    let a2 = vec![10000000 * 10000000, 100000000 * 100000000];
    testing(a1, a2, false);
    let a1 = vec![2, 2, 3];
    let a2 = vec![4, 9, 9];
    testing(a1, a2, false);
    let a1 = vec![-121, -144, 19, -161, 19, -144, 19, -11];
    let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, true);
    
    testing(vec![494, 96, 694, 48, 25, 701, 704], vec![9216, 495616, 2304, 244036, 625, 491401, 481636], true);
    testing(vec![150, 14, 571, 198, 402, 562, 85], vec![161605, 22500, 39204, 161604, 7225, 326041, 196, 315844], false);
    testing(vec![307, 772, 190, 79, 677, 279, 511], vec![261121, 36100, 94249, 6241, 595984, 458329, 77841], true);
    testing(vec![103, 71, 24, 653, 52], vec![10609, 576, 426409, 5041, 2704], true);
    testing(vec![609, 265, 223, 552, 437, 227, 374], vec![190969, 304704, 139876, 49729, 51529, 70225, 370881], true);
    testing(vec![453, 433, 623, 184, 761, 585, 523, 321], vec![33856, 273529, 205209, 103041, 579121, 342225, 388129, 187489], true);
    testing(vec![772, 603, 736, 754, 701, 298, 451], vec![363610, 88804, 595984, 203401, 541696, 363609, 491401, 568516], false);
    testing(vec![334, 710, 731, 41, 557, 395, 307], vec![156026, 534361, 1681, 94249, 111556, 504100, 156025, 310249], false);
    testing(vec![1, 2, 3, 4, 5], vec![1, 4, 9, 25, 32], false);
    testing(vec![259, 337, 351, 438, 592], vec![113569, 191844, 123202, 123201, 350464, 67081], false);
    testing(vec![190, 473, 426, 216, 165, 111], vec![181476, 12321, 223729, 27225, 36100, 46656], true);
    testing(vec![681, 660, 72, 399, 51, 431, 499, 211], vec![2601, 44521, 5184, 159201, 185761, 249001, 435600, 463761, 44522], false);
    testing(vec![366, 743, 148, 553, 452, 597], vec![133956, 356409, 552049, 305809, 21904, 204304], true);
    testing(vec![614, 431, 499, 681, 330, 123], vec![15129, 185761, 108900, 463761, 376996, 249001], true);
    testing(vec![99, 151, 21, 376, 334, 85, 575, 677], vec![9801, 441, 7225, 141377, 458329, 141376, 330625, 22801, 111556], false);
    testing(vec![451, 83, 377, 471, 122], vec![203401, 6889, 221841, 14884, 142129], true);
    testing(vec![73, 209, 615, 295, 415, 272, 3], vec![87025, 73984, 9, 5329, 43681, 378225, 172225], true);
    testing(vec![723, 412, 214, 771, 297, 763, 632, 223, 713], vec![399424, 522729, 594441, 45796, 508369, 88209, 582169, 169744, 49729, 399425], false);
    testing(vec![156, 192, 9, 104, 597, 190, 276, 417, 134], vec![24336, 36864, 173889, 356409, 76176, 36100, 81, 17956, 10816], true);

    let a1 = vec![4, 4];
    let a2 = vec![1, 31];
    testing(a1, a2, false);

    testing(vec![7, 2, 9, 6, 4], vec![49, 4, 81, 36, 16], true);
    testing(vec![9, 6, 1, 5, 2, 5], vec![81, 36, 1, 25, 4, 25, 26], false);
    testing(vec![2, 1, 8, 9, 10, 10, 4, 3, 9], vec![4, 1, 64, 81, 100, 100, 16, 9, 81], true);
    testing(vec![6, 7, 8, 9, 7, 5, 4, 10], vec![36, 49, 64, 81, 49, 25, 16, 100], true);
    testing(vec![1, 4, 2, 6, 5, 7, 5, 7], vec![1, 16, 4, 36, 25, 49, 25, 49, 50], false);
    testing(vec![1, 6, 8, 1, 9], vec![1, 36, 64, 1, 81, 82], false);
    testing(vec![10, 2, 6, 10, 3, 1, 2], vec![100, 4, 36, 100, 9, 1, 4, 5], false);
    testing(vec![3, 5, 8, 5, 7, 8], vec![9, 25, 64, 25, 49, 64, 65], false);
    testing(vec![1, 5, 4, 3, 6], vec![1, 25, 16, 9, 36], true);
    testing(vec![5, 10, 5, 10, 1, 9, 10, 2, 7], vec![25, 100, 25, 100, 1, 81, 100, 4, 49, 50], false);
    testing(vec![5, 5, 5, 2, 10, 4, 7, 5, 10], vec![25, 25, 25, 4, 100, 16, 49, 25, 100, 101], false);
    testing(vec![8, 7, 1, 3, 10, 9, 3], vec![64, 49, 1, 9, 100, 81, 9], true);
    testing(vec![5, 1, 1, 4, 7, 1], vec![25, 1, 1, 16, 49, 1], true);
    testing(vec![10, 4, 7, 1, 7, 8, 9], vec![100, 16, 49, 1, 49, 64, 81, 82], false);
    testing(vec![1, 3, 1, 4, 10, 4], vec![1, 9, 1, 16, 100, 16], true);
    testing(vec![5, 4, 1, 8, 6], vec![25, 16, 1, 64, 36, 37], false);
    testing(vec![2, 5, 3, 1, 8, 10, 9, 10, 5], vec![4, 25, 9, 1, 64, 100, 81, 100, 25], true);
    testing(vec![7, 2, 8, 10, 1, 2, 1], vec![49, 4, 64, 100, 1, 4, 1], true);
    testing(vec![5, 2, 9, 6, 10, 2, 3], vec![25, 4, 81, 36, 100, 4, 9], true);
    testing(vec![-4, -5, -7, -3, -3, -5, -10, -9], vec![16, 25, 49, 9, 9, 25, 100, 81], true);
    
    let a1 = vec![3, 4];
    let a2 = vec![0, 25];
    testing(a1, a2, false);
}
