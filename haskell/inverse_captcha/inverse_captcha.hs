import System.Environment
import Data.Char (digitToInt)
import Data.List (transpose)

captcha :: [String] -> [Int]
captcha args =
  map digitToInt $ head args

cmd :: [String] -> String
cmd args =
  head $ tail args

shiftn :: String -> [Int] -> Int
shiftn cmd captcha
  | cmd == "wrap" = 1
  | cmd == "half" = length captcha `div` 2
  | otherwise = 1

captchaPairs :: [String] -> [[Int]]
captchaPairs args =
  transpose [captcha args, drop n (captcha args) ++ take n (captcha args)]
    where n = shiftn (cmd args) (captcha args)

main = do
  args <- getArgs
  print (sum[head p | p <- captchaPairs args, head p == last p])
