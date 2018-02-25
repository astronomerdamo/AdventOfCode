import Data.List (nub, sort)
import System.Environment
import System.IO

decompsort :: String -> [[String]]
decompsort x = [[sort b | b <- a] | a <- map words (lines x)]

main = do
  args <- getArgs
  contents <- readFile $ head args
  print
    (length [x | x <- decompsort contents, length x == length (nub x)])
