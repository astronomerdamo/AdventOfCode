import Data.List (nub, sort)
import System.Environment
import System.IO

decompsort :: String -> [[String]]
decompsort x = [[sort b | b <- a] | a <- map words (lines x)]

main = do
  args <- getArgs
  infile <- openFile (head args) ReadMode
  contents <- hGetContents infile
  print
    (length [x | x <- decompsort contents, length x == length (nub x)])
