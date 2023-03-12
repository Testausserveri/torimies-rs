-- stack script --resolver lts-20 --package sqlite-simple --package text

{-# LANGUAGE OverloadedStrings #-}

import qualified Data.Text as T
import           Database.SQLite.Simple
import Control.Monad
import Data.List

vahtiQuery :: String -> [(Int, String)]
vahtiQuery q = [(1, "https://www.tori.fi/koko_suomi?q=" ++ q), (2, "https://www.huuto.net/haku?words=" ++ q)]

wordList = ["thinkpad", "lenovo", "xeon", "server", "hp", "elitebook", "i3", "i5", "i7"]

generate :: [[(Int, String)]]
generate = do
    ws <- filter (not . null) $ filterM (const [True, False]) wordList
    return . vahtiQuery $ intercalate "+" ws

main :: IO ()
main = do
    putStrLn "Generating Vahtis..."
    let vahtis = concat generate

    putStrLn $ "Inserting " ++ show (length vahtis) ++ " Vahtis.."
    conn <- open "test.sqlite"
    mapM_ (execute conn "INSERT INTO Vahdit (site_id, user_id, last_updated, url, delivery_method) VALUES (?, 328625071327412267, 0, ?, 1)") vahtis
