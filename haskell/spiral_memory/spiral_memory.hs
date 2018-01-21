import System.Environment
import Text.Printf

nring :: Int -> Int
nring n =
  ceiling $ (sqrt (fromIntegral n) - 1) / 2

mdist :: Int -> Int
mdist n =
  abs (((n - 1) `mod` (2 * r)) - r) + r
    where r = nring n

main = do
  args <- getArgs
  let n = read . head $ args :: Int
  -- Find the manhatten distance
  printf "Manhattan Distance: %d\n" (mdist n)
