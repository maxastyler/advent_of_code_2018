import Data.List.Split

main :: IO ()
main = do
	file_str <- readFile "input.txt"
	let nums_wrong = splitOn "\n" file_str
	let nums = map convert_num $ take (length nums_wrong - 1) nums_wrong
	print $ foldl (+) 0 $ nums
	let repeating = scanl (+) 0 $ cycle nums
	print $ get_first_repetition repeating

convert_num :: String -> Integer
convert_num ('+':x) = read x
convert_num x = read x

get_first_repetition :: (Eq a) => [a] -> a
get_first_repetition scanned = fst $ head $ filter (\x -> (fst x) `elem` (snd x) ) (list_pairs scanned)
	where list_pairs l = [(l !! i, take i l) | i <- [2..]]
