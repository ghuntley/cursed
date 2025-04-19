-- FASTA benchmark for Haskell

import System.IO
import System.CPUTime
import Data.Time.Clock
import Control.Monad
import Text.Printf
import Data.List
import System.Random

-- Constants for the random number generator
seed :: Int
seed = 42

im :: Int
im = 139968

ia :: Int
ia = 3877

ic :: Int
ic = 29573

-- Define DNA sequences
alu :: String
alu = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA"

iubProb :: [(Char, Double)]
iubProb = zip ['a', 'c', 'g', 't', 'B', 'D', 'H', 'K', 'M', 'N', 'R', 'S', 'V', 'W', 'Y']
              [0.27, 0.12, 0.12, 0.27, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02]

homoSapiensProb :: [(Char, Double)]
homoSapiensProb = zip ['a', 'c', 'g', 't']
                      [0.3029549426680, 0.1979883004921, 0.1975473066391, 0.3015094502008]

-- Generate a random number
genRandom :: Int -> (Int, Double)
genRandom s = (s', fromIntegral s' / fromIntegral im)
  where s' = (s * ia + ic) `mod` im

-- Generate a random FASTA sequence
genRandomFasta :: Int -> Int -> [(Char, Double)] -> (Int, String)
genRandomFasta n s probs = foldl' acc (s, "") [1..n]
  where
    acc (s', result) _ = 
      let (s'', r) = genRandom s'
          (c, _) = head $ dropWhile (\(_, p) -> r >= p) $
                   scanl' (\(c', p') (c'', p'') -> (c'', p' + p'')) (head probs) (tail probs)
      in (s'', result ++ [c])

-- Repeat a sequence until it reaches the required length
repeatFasta :: Int -> String -> String
repeatFasta n seq = take n $ cycle seq

main :: IO ()
main = do
    let n = 1000000  -- Default sequence length
    start <- getCurrentTime

    -- Write FASTA header and sequence for Homo sapiens Alu
    putStrLn ">ONE Homo sapiens alu"
    let aluSeq = repeatFasta n alu
    putStrLn aluSeq

    -- Write FASTA header and random sequence for IUB ambiguity codes
    putStrLn ">TWO IUB ambiguity codes"
    let (newSeed, iubSeq) = genRandomFasta n seed iubProb
    putStrLn iubSeq

    -- Write FASTA header and random sequence for Homo sapiens frequency
    putStrLn ">THREE Homo sapiens frequency"
    let (_, sapiensSeq) = genRandomFasta n newSeed homoSapiensProb
    putStrLn sapiensSeq

    end <- getCurrentTime
    let diff = diffUTCTime end start
    putStrLn $ "Time taken: " ++ show (diff * 1000) ++ " ms"

    -- Calculate approximate memory usage
    let memoryUsage = length aluSeq + length iubSeq + length sapiensSeq
    putStrLn $ "Memory used: " ++ show (memoryUsage `div` 1024) ++ " KB"