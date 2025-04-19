-- Binary trees benchmark for Haskell

import System.Environment
import System.IO
import Control.Concurrent
import Control.Parallel
import Control.Monad
import Data.Time.Clock
import Text.Printf

data Tree = Node !Int !Tree !Tree | Empty

-- Create a binary tree of a given depth
makeTree :: Int -> Tree
makeTree depth
    | depth <= 0 = Node 1 Empty Empty
    | otherwise  = Node 1 (makeTree (depth - 1)) (makeTree (depth - 1))

-- Check a tree by calculating its item sum
checkTree :: Tree -> Int
checkTree Empty = 0
checkTree (Node item left right) = item + checkTree left + checkTree right

-- Process a tree of given depth
iterateTree :: Int -> Int
iterateTree depth = checkTree $ makeTree depth

-- Create and check a bunch of trees
mainProcess :: Int -> Int -> Int -> IO ()
mainProcess minDepth maxDepth stretchDepth = do
    -- Create and check a stretch tree
    let stretchTree = makeTree stretchDepth
    let stretchCheck = checkTree stretchTree
    printf "stretch tree of depth %d\t check: %d\n" stretchDepth stretchCheck

    -- Create and check a long-lived tree
    let longLivedTree = makeTree maxDepth
    
    -- Create and check multiple trees of increasing depth
    let depths = [minDepth, minDepth + 2 .. maxDepth]
    mapM_ (\depth -> do
        let iterations = 2 ^ (maxDepth - depth + minDepth)
        let check = sum [iterateTree depth | _ <- [1..iterations]]
        printf "%d\t trees of depth %d\t check: %d\n" iterations depth check
        ) depths
    
    -- Check the long-lived tree at the end
    let longLivedCheck = checkTree longLivedTree
    printf "long lived tree of depth %d\t check: %d\n" maxDepth longLivedCheck

main :: IO ()
main = do
    -- Define tree depths
    let n = 10  -- Default value
    let minDepth = 4
    let maxDepth = max (minDepth + 2) n
    let stretchDepth = maxDepth + 1
    
    startTime <- getCurrentTime
    
    -- Process the trees
    mainProcess minDepth maxDepth stretchDepth
    
    endTime <- getCurrentTime
    let diff = diffUTCTime endTime startTime
    printf "Time taken: %s ms\n" (show (diff * 1000))