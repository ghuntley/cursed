-- N-body simulation benchmark for Haskell

import System.Environment
import System.IO
import Control.Monad
import Data.Time.Clock
import Text.Printf
import qualified Data.Vector as V
import qualified Data.Vector.Mutable as MV

-- Constants
pi' :: Double
pi' = 3.141592653589793

solarMass :: Double
solarMass = 4.0 * pi' * pi'

daysPerYear :: Double
daysPerYear = 365.24

-- Planet data structure
data Planet = Planet {
    x :: !Double,
    y :: !Double,
    z :: !Double,
    vx :: !Double,
    vy :: !Double,
    vz :: !Double,
    mass :: !Double
} deriving (Show)

-- Initialize the solar system
initSolarSystem :: V.Vector Planet
initSolarSystem = V.fromList [
    -- Sun
    Planet {
        x = 0.0,
        y = 0.0,
        z = 0.0,
        vx = 0.0,
        vy = 0.0,
        vz = 0.0,
        mass = solarMass
    },
    -- Jupiter
    Planet {
        x = 4.84143144246472090e+00,
        y = -1.16032004402742839e+00,
        z = -1.03622044471123109e-01,
        vx = 1.66007664274403694e-03 * daysPerYear,
        vy = 7.69901118419740425e-03 * daysPerYear,
        vz = -6.90460016972063023e-05 * daysPerYear,
        mass = 9.54791938424326609e-04 * solarMass
    },
    -- Saturn
    Planet {
        x = 8.34336671824457987e+00,
        y = 4.12479856412430479e+00,
        z = -4.03523417114321381e-01,
        vx = -2.76742510726862411e-03 * daysPerYear,
        vy = 4.99852801234917238e-03 * daysPerYear,
        vz = 2.30417297573763929e-05 * daysPerYear,
        mass = 2.85885980666130812e-04 * solarMass
    },
    -- Uranus
    Planet {
        x = 1.28943695621391310e+01,
        y = -1.51111514016986312e+01,
        z = -2.23307578892655734e-01,
        vx = 2.96460137564761618e-03 * daysPerYear,
        vy = 2.37847173959480950e-03 * daysPerYear,
        vz = -2.96589568540237556e-05 * daysPerYear,
        mass = 4.36624404335156298e-05 * solarMass
    },
    -- Neptune
    Planet {
        x = 1.53796971148509165e+01,
        y = -2.59193146099879641e+01,
        z = 1.79258772950371181e-01,
        vx = 2.68067772490389322e-03 * daysPerYear,
        vy = 1.62824170038242295e-03 * daysPerYear,
        vz = -9.51592254519715870e-05 * daysPerYear,
        mass = 5.15138902046611451e-05 * solarMass
    }
  ]

-- Offset the momentum of the system
offsetMomentum :: V.Vector Planet -> V.Vector Planet
offsetMomentum bodies = 
    let (px, py, pz) = V.foldl' (\(px', py', pz') p -> 
                            (px' + vx p * mass p, 
                             py' + vy p * mass p, 
                             pz' + vz p * mass p)) (0.0, 0.0, 0.0) bodies
        sun = bodies V.! 0
        newSun = sun { vx = -px / solarMass, 
                       vy = -py / solarMass, 
                       vz = -pz / solarMass }
    in bodies V.// [(0, newSun)]

-- Calculate the energy of the system
energy :: V.Vector Planet -> Double
energy bodies = 
    let n = V.length bodies
        bodyEnergy i = 
            let b = bodies V.! i
                kineticEnergy = 0.5 * mass b * (vx b * vx b + vy b * vy b + vz b * vz b)
                potentialEnergy = V.sum $ V.imap (\j b2 -> 
                                    if j > i then
                                        let dx = x b - x b2
                                            dy = y b - y b2
                                            dz = z b - z b2
                                            distance = sqrt (dx*dx + dy*dy + dz*dz)
                                        in -((mass b * mass b2) / distance)
                                    else
                                        0.0) bodies
            in kineticEnergy + potentialEnergy
    in V.sum $ V.generate n bodyEnergy

-- Advance the system by dt
advance :: Double -> V.Vector Planet -> V.Vector Planet
advance dt bodies = V.create $ do
    let n = V.length bodies
    result <- V.thaw bodies
    
    -- Calculate interactions
    forM_ [0..n-1] $ \i -> do
        bi <- MV.read result i
        forM_ [i+1..n-1] $ \j -> do
            bj <- MV.read result j
            let dx = x bi - x bj
                dy = y bi - y bj
                dz = z bi - z bj
                distance = sqrt (dx*dx + dy*dy + dz*dz)
                mag = dt / (distance * distance * distance)
                bi_mass_mag = mass bi * mag
                bj_mass_mag = mass bj * mag
                
                bi' = bi { vx = vx bi - dx * bj_mass_mag,
                           vy = vy bi - dy * bj_mass_mag,
                           vz = vz bi - dz * bj_mass_mag }
                
                bj' = bj { vx = vx bj + dx * bi_mass_mag,
                           vy = vy bj + dy * bi_mass_mag,
                           vz = vz bj + dz * bi_mass_mag }
            
            MV.write result i bi'
            MV.write result j bj'
    
    -- Update positions
    forM_ [0..n-1] $ \i -> do
        b <- MV.read result i
        let b' = b { x = x b + dt * vx b,
                     y = y b + dt * vy b,
                     z = z b + dt * vz b }
        MV.write result i b'
    
    return result

-- Main function
main :: IO ()
main = do
    let n = 1000000  -- Number of iterations
    let bodies = offsetMomentum initSolarSystem
    
    startTime <- getCurrentTime
    
    let initialEnergy = energy bodies
    printf "Initial energy: %.9f\n" initialEnergy
    
    -- Run the simulation
    let finalBodies = iterate (advance 0.01) bodies !! n
    let finalEnergy = energy finalBodies
    
    printf "Final energy: %.9f\n" finalEnergy
    printf "Energy delta: %.9f\n" (finalEnergy - initialEnergy)
    
    endTime <- getCurrentTime
    let diff = diffUTCTime endTime startTime
    printf "Time taken: %s ms\n" (show (diff * 1000))