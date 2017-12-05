import System.Environment
import Data.Char (digitToInt)
import Data.List (transpose)

shiftn :: String -> [Int] -> Int
shiftn cmd captcha =
  if cmd == "wrap" then 1
  else if cmd == "half" then (length captcha) `div` 2
  else 1

main = do
  args <- getArgs
  let captcha = (map digitToInt (head args))
  let cmd = head (tail args)
  let n = shiftn cmd captcha
  let captcha_pairs = transpose [captcha, drop n captcha ++ take n captcha]
  print (sum[head p | p <- captcha_pairs, (head p == last p)])
