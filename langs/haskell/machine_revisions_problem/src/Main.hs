module Main where

main :: IO ()
main = putStrLn "Hello, Haskell!"



machines :: (Num n) => [(String, [(n, n)])]
machines = [
        ("m1",  [( 8, 15), ( 2, 20), ( 2, 30)]),
        ("m2",  [( 7, 20), ( 3, 30), ( 2, 50)]),
        ("m3",  [( 6, 25), ( 4, 35), ( 2, 55)]),
        ("m4",  [( 8, 15), ( 2, 20), ( 2, 30)]),
        ("m5",  [( 8, 12), ( 2, 15), ( 2, 28)]),
        ("m6",  [( 8, 15), ( 2, 20), ( 2, 30)]),
        ("m7",  [( 7, 20), ( 3, 30), ( 2, 50)]),
        ("m8",  [( 6, 25), ( 4, 35), ( 2, 55)]),
        ("m9",  [( 8, 15), ( 2, 20), ( 2, 30)]),
        ("m10", [( 8, 12), ( 2, 15), ( 2, 28)]),
        ("m11", [( 8, 15), ( 2, 20), ( 2, 30)]),
        ("m12", [( 8, 12), ( 2, 15), ( 2, 28)])]


-- getMachineVisits lmachines = foldl (\acc (n, _) -> acc + n) 0 visit_list
-- getMachineVisits = sum $ snd $ sum fst