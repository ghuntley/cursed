-- Mandelbrot set calculation benchmark for Haskell

import System.Environment
import System.IO
import Control.Monad
import Data.Time.Clock
import Text.Printf
import Data.Complex
import qualified Data.Vector as V
import qualified Data.Vector.Unboxed as UV

-- Constants
width, height :: Int
width = 800
height = 800

maxIterations :: Int
maxIterations = 100

-- Check if a point belongs to the Mandelbrot set
isInMandelbrot :: Complex Double -> Int -> Int
isInMandelbrot c maxIter = go 0 (0 :+ 0)
  where
    go i z
      | i >= maxIter = maxIter
      | magnitude z > 2 = i
      | otherwise = go (i + 1) (z*z + c)

-- Calculate the entire Mandelbrot set
computeMandelbrot :: Int -> V.Vector (UV.Vector Int)
computeMandelbrot maxIter = V.generate height $ \y ->
  UV.generate width $ \x -> isInMandelbrot (toComplex x y) maxIter
  where
    toComplex x y = (fromIntegral x - fromIntegral width / 2) * 4 / fromIntegral width :+
                    (fromIntegral y - fromIntegral height / 2) * 4 / fromIntegral height

-- Count non-black pixels
countNonBlack :: V.Vector (UV.Vector Int) -> Int -> Int
countNonBlack result maxIter = V.sum $ V.map (UV.length . UV.filter (< maxIter)) result

-- Main function
main :: IO ()
main = do
    startTime <- getCurrentTime
    
    -- Compute the Mandelbrot set
    let result = computeMandelbrot maxIterations
    let nonBlackCount = countNonBlack result maxIterations
    
    putStrLn $ "Mandelbrot set calculation finished."
    printf "Image size: %d x %d\n" width height
    printf "Maximum iterations: %d\n" maxIterations
    printf "Non-black pixels: %d\n" nonBlackCount
    
    endTime <- getCurrentTime
    let diff = diffUTCTime endTime startTime
    printf "Time taken: %s ms\n" (show (diff * 1000))