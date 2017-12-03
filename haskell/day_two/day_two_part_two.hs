import System.Environment
import System.IO

rChar :: String -> Int
rChar x = read x::Int

ckevendiv :: [Int] -> Int
ckevendiv x = foldl (+) 0 [a `div` b | a <- x, b <- x, a `mod` b == 0 && a /= b]

main = do
  args <- getArgs
  infile <- openFile (head args) ReadMode
  contents <- hGetContents infile
  let spreadsheet = [[rChar y | y <- x] | x <- map words (lines contents)]
  print (sum [ckevendiv x | x <- spreadsheet])
