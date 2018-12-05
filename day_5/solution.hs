import Data.Char

main = do
  file_str <- readFile "input.txt"
  print $ length $ filter (/= '\n') $ react_iteratively file_str

react_elements (a:rest@(b:c)) | same_letter && opposite_case = react_elements c
                              | otherwise = a:(react_elements rest)
          where pol = flip elem ['a'..'z']
                same_letter = toUpper a == toUpper b
                opposite_case = pol a /= pol b
react_elements a = a

react_iteratively s | s == s' = s
                    | otherwise = react_iteratively s'
                    where s' = react_elements s
