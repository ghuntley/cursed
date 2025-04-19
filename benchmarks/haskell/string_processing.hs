-- String processing benchmark for Haskell

import System.Environment
import System.IO
import Control.Monad
import Data.Time.Clock
import Text.Printf
import Data.Char
import System.Random
import qualified Data.Text as T

-- Create a random string of specified length
createRandomString :: Int -> IO String
createRandomString length = do
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    indices <- replicateM length $ randomRIO (0, length chars - 1)
    return $ map (chars !!) indices

-- Process a string with various operations
processString :: String -> String
processString input = do
    -- Replace all vowels with uppercase version
    let replaceVowels c = case c of
            'a' -> 'A'
            'e' -> 'E'
            'i' -> 'I'
            'o' -> 'O'
            'u' -> 'U'
            _ -> c
    
    let replaced = map replaceVowels input
    
    -- Replace digits with doubled value
    let replaceDigits c = if isDigit c
                          then let digit = digitToInt c
                               in head (show (digit * 2))
                          else c
    
    let replacedDigits = map replaceDigits replaced
    
    -- Capitalize first letter if string is not empty
    let capitalized = case replacedDigits of
            [] -> []
            (c:cs) -> toUpper c : cs
    
    -- Reverse the string
    let reversed = reverse capitalized
    
    -- Take first half
    take (length reversed `div` 2) reversed

-- Process multiple strings of different sizes
processStrings :: Int -> Int -> IO String
processStrings count size = do
    strings <- replicateM count $ createRandomString size
    return $ concatMap processString strings

-- Main function
main :: IO ()
main = do
    startTime <- getCurrentTime
    
    -- Process strings of different sizes
    small <- processStrings 10000 10    -- 10,000 strings of length 10
    medium <- processStrings 1000 100   -- 1,000 strings of length 100
    large <- processStrings 100 1000    -- 100 strings of length 1,000
    
    let resultLength = length small + length medium + length large
    printf "Processed string length: %d\n" resultLength
    
    endTime <- getCurrentTime
    let diff = diffUTCTime endTime startTime
    printf "Time taken: %s ms\n" (show (diff * 1000))