import System.Environment
import System.IO

rChar :: String -> Int
rChar x = read x::Int

cksmln :: [Int] -> Int
cksmln x = maximum x - minimum x

main = do
  args <- getArgs
  contents <- readFile $ head args
  let spreadsheet = [[rChar y | y <- x] | x <- map words (lines contents)]
  print (sum [cksmln x | x <- spreadsheet])
