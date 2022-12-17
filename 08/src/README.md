Strategy:
=========

```
F<num> = Free sight to outside the forest of length <num>
B<num> = Blocked sight of length <num>
?      = Not yet investigated

          (0, 0)    (0, 1)    (3, 2)    (1, 2)    (2, 2)
30373     3....     .0...     .....     .....     .....
25512     .....     .....     .....     ..5..     .....
65332     .....     .....     .....     .....     ..3..
33549     .....     .....     ..5..     .....     .....
35390     .....     .....     .....     .....     .....
          N: F0     N: F0     N: 2      N: F0     N: 1
          E: B2     E: 1      E: 2      E: F0     E: 1
          S: B2     S: 1      S: 1      S: 2      S: 1
          W: F0     W: 1      W: 2      W: 1      W: 1
```

Set outer border trees to free sight in applicable directions. e.g.
(0, 0) -> N,W: F0, E,S:?
(0, 1) -> N: F0, W,E,S: ?
(4, 4) -> N,W: ?, E,S: F0

scan per grid line, with recipe:
```
- Take next tree T
- T.east already set? Then continue with next tree.
- Take tree R to the right of tree T
    - No tree to the right?    -> Make T.east = F0
                               -> End scan

    - R >= T?                  -> make T.east = B1
                               -> Continue with next

    - R < T?                   -> first resolve R
                               -> then make T.east = R.east + 1
                               -> Continue with next
```

Trial on the above grid, row 0 (30373)

```
(0, 0).west = F0
(0, 4).east = F0

(0, 0) R < T                   -> first resolve (0, 1)
    (0, 1) R >= T              -> (0, 1).east = B1
(0, 0) R resolved              -> (0, 0).east = (0, 1).east + 1 = B1 + 1 = B2
                               -> Continue with next
(0, 1) Aready resolved         -> Continue with next    
(0, 2) R >= T                  -> (0, 2).east = B1
                               -> Continue with next
(0, 3) R < T                   -> first resolve (0, 4)
    (0, 4)                     -> (0, 4).east already resolved
(0, 3) R resolved              -> (0, 3).east = (0, 4).east + 1 = F0 + 1 = F1
                                  Continue with next
(0, 4) Already resolved        -> Continue with next
No next, end scan.                                 
```

Resulting eastward views:
```
3  0  3  7  3
B2 B1 B1 F1 F0
```

Expected westward view (in order of appearance):
```
3  7  3  0  3
B1 F3 B2 B1 F0
            F0
B1          F0
B1       B1 F0
B1    B2 B1 F0
B1 B3 B2 B1 F0
```
Darn, thinko error.
7 should be F3, because it's higher than the following 3 0 3 heights.