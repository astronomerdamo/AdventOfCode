import System.Environment
import Text.Printf

nring :: Int -> Int
nring n =
  ceiling $ (sqrt (fromIntegral n) - 1) / 2

mdist :: Int -> Int
mdist n =
  abs (((n - 1) `mod` (2 * r)) - r) + r
    where r = nring n

rcoords :: Int -> [Int]
rcoords r =
  [-(r-1)..r] ++ replicate (2*r) r ++ reverse [-r..(r-1)] ++ replicate (2*r) (-1*r)

crec :: Int -> ((Int, Int), Int)
crec 1 = ((0,0),1)
crec n = (head $ zip x y, n)
    where r = nring n
          coords = rcoords r
          x = drop (2*r) coords ++ take (2*r) coords
          y = coords

main = do
  args <- getArgs
  let n = read . head $ args :: Int
  let alist = map crec [1..n]
  print alist
  -- Find the manhatten distance
  printf "Manhattan Distance: %d\n" (mdist n)
