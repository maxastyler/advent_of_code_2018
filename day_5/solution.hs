import Data.Char

main = do
  file_str <- readFile "input.txt"
  let filtered = filter (/='\n') file_str
  print $ length $ react_foldr filtered
  print $ minimum [length $ react_foldr $ filter (\x -> toLower x /= l) filtered | l <- ['a'..'z']]

react_elements (a:rest@(b:c)) | same_letter && a /= b = react_elements c
                              | otherwise = a:(react_elements rest)
                where same_letter = toUpper a == toUpper b
react_elements a = a

react_iteratively s | s == s' = s
                    | otherwise = react_iteratively s'
                    where s' = react_elements s

react_foldr :: [Char] -> [Char]
react_foldr = foldr react ""
  where react a (b:c) | a /= b && toUpper a == toUpper b = c
        react a c = a:c
