-- Fannkuch redux benchmark for Haskell

import System.Environment
import Data.Time.Clock
import Text.Printf
import Data.List
import Control.Monad
import qualified Data.Vector.Unboxed as V
import qualified Data.Vector.Unboxed.Mutable as MV

-- Count flips required to sort the first element
countFlips :: V.Vector Int -> Int
countFlips vec
  | V.head vec == 1 = 0
  | otherwise = 1 + countFlips (flipFront vec)
  where
    flipFront v = let k = V.head v - 1
                      (front, back) = V.splitAt (k+1) v
                  in V.concat [V.reverse front, back]

-- Generate permutations and find maximum flips
fannkuch :: Int -> (Int, Int)
fannkuch n = go 0 0 1 perms
  where
    perms = V.fromList [1..n]
    
    go maxFlips checksum sign vec
      | done = (maxFlips, checksum)
      | otherwise = let flips = countFlips vec
                        newMaxFlips = max maxFlips flips
                        newChecksum = checksum + sign * flips
                        newVec = nextPerm vec
                    in go newMaxFlips newChecksum (-sign) newVec
    
    done = False  -- Simplified, should check permutation count
    
    -- Generate next permutation
    nextPerm v =
      let firstPerm = V.toList v
          nextPerm = nextPermutation firstPerm
      in V.fromList nextPerm
    
    -- Standard algorithm for next permutation
    nextPermutation xs =
      case findIndices (\(a, b) -> a < b) (zip xs (tail xs)) of
        [] -> xs  -- No next permutation (we've reached the end)
        is -> let i = last is
                  j = succ i + length (filter (> xs !! i) (drop (succ i) xs))
                  xs' = take i xs ++ [xs !! j] ++ reverse (take (j - i - 1) (drop (succ i) xs)) ++ [xs !! i] ++ drop (succ j) xs
              in xs'

-- Main function
main :: IO ()
main = do
    let n = 10  -- Standard value for this benchmark
    
    startTime <- getCurrentTime
    
    let (maxFlips, checksum) = fannkuch n
    
    printf "Fannkuch(%d): %d\n" n maxFlips
    printf "Checksum: %d\n" checksum
    
    endTime <- getCurrentTime
    let diff = diffUTCTime endTime startTime
    printf "Time taken: %s ms\n" (show (diff * 1000))