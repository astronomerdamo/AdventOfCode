import System.Environment
import Text.Printf

-- | Part A Solution
-- | nring   -> finds the ring that n belongs to
-- | lring   -> finds the odd root of the previous ring
-- | rcoords -> creates the basic x,y cell coordinates
-- | crec    -> creates the associated list
-- | mdist*  -> finds the manhattan distance

nring :: Int -> Int
nring n =
  ceiling $ (sqrt (fromIntegral n) - 1) / 2

lring :: Int -> Int
lring n
  | odd flroot = flroot
  | otherwise  = flroot - 1
    where flroot = floor $ sqrt (fromIntegral n - 1)

-- Analytic solution
mdist :: Int -> Int
mdist n =
  abs (((n - 1) `mod` (2 * r)) - r) + r
    where r = nring n

-- AssocList solution
mdistalist :: [((Int, Int), Int)] -> Int
mdistalist xs =
  abs x + abs y
    where ((x,y),_) = last xs

rcoords :: Int -> [Int]
rcoords r =
  [-(r-1)..r] ++ replicate (2*r) r ++ reverse [-r..(r-1)] ++ replicate (2*r) (-1*r)

crec :: ((Int, Int), Int) -> ((Int, Int), Int)
crec ((_,_),1) = ((0,0),1)
crec ((x,y),n) = (zip x y !! (n - 1 - lring n ^ 2), n)
    where coords = rcoords $ nring n
          x = drop (2 * nring n) coords ++ take (2 * nring n) coords
          y = coords

-- | Part B Solution
-- | csrec -> creates the summed associated list
csrec :: ((Int, Int), Int) -> [((Int, Int), Int)] -> [((Int, Int), Int)]
csrec (tk, v) acc =
  (tk,
   sum (map snd (filter (\(fk, _) -> fk `elem` memcoords tk) acc))
  ) : acc
    where memcoords (x,y) = [(x-1, y-1), (x-1, y), (x-1, y+1), (x+1, y-1), (x+1, y), (x+1, y+1), (x, y-1), (x, y+1)]

main = do
  args <- getArgs
  let n = read . head $ args :: Int
  let alist = map (crec . (\x -> ((0,0),x))) [1..n]
  -- Find the manhatten distance
  printf "Manhattan Distance: %d\n" (mdistalist alist)
  let salist = takeWhile (\(k,v) -> v < (n + n `div` 2)) . reverse . foldr csrec [((0,0),1)] $ reverse (drop 1 alist)
  printf "First summed value larger than %d: %d\n" n (snd . head $ dropWhile (\(k,v) -> v < n) salist)
