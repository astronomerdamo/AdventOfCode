import Data.List (nub)
import System.Environment
import System.IO

decomp :: String -> [[String]]
decomp x = map words (lines x)

main = do
  args <- getArgs
  contents <- readFile $ head args
  print
    (length [x | x <- decomp contents, length x == length (nub x)])
