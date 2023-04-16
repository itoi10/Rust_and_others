
-- ２倍
doubleMe x = x + x

-- 引数２つを受け取って２倍したものを足す
doubleUs x y = x * 2 + y * 2

-- 引数が100より大きい場合はそのまま返す. そうでない場合は２倍して返す
doubleSmallNumber x =
  if x > 100
    then x
    else x * 2

-- main = print (doubleSmallNumber 99)

doubleSmallNumber' x = (if x > 100 then x else x * 2) + 1

-- 各辺の長さは整数で10以下, 周囲の長さは24の直角三角形を見つける
rightTriangles = [(a, b, c) | c <- [1 .. 10], a <- [1 .. c], b <- [1 .. a], a ^ 2 + b ^ 2 == c ^ 2, a + b + c == 24]

factorial :: Integer -> Integer
factorial n = product [1 .. n]

-- Float
circumference :: Float -> Float
circumference r = 2 * pi * r

-- Double
circumference' :: Double -> Double
circumference' r = 2 * pi * r

-- func ptn
lucky :: Int -> String
lucky 7 = "seven!"
lucky x = "not !"

-- head
head' :: [a] -> a
head' [] = error "Can't call head on an empty list"
head' (x : _) = x

-- bmi
bmiTell :: Double -> Double -> String
bmiTell weight height
  | bmi <= skinny = "underweight"
  | bmi <= normal = "normal"
  | bmi <= fat = "fat"
  | otherwise = "whale"
  where
    bmi = weight / height ^ 2
    skinny = 18.5
    normal = 25.0
    fat = 30.0

-- let bmi
calcBmis :: [(Double, Double)] -> [Double]
calcBmis xs = [bmi | (w, h) <- xs, let bmi = w / h ^ 2]

-- initials
initials :: String -> String -> String
initials first_name last_name = [f] ++ ". " ++ [l] ++ "."
  where
    (f : _) = first_name
    (l : _) = last_name

-- let sample
cylinder :: Double -> Double -> Double
cylinder r h =
  let sideArea = 2 * pi * r * h
      topArea = pi * r ^ 2
   in sideArea + 2 * topArea

-- case sample
headCase :: [a] -> a
headCase xs = case xs of
  [] -> error "No headCase for empty lists"
  (x : _) -> x

-- 再帰
maximum' :: (Ord a) => [a] -> a
maximum' [] = error "maximum of empty list" -- 空のリスト
maximum' [x] = x -- 長さ1のリスト
maximum' (x : xs) = max x (maximum' xs) -- その他のリスト

-- 再帰2
replicate' :: Int -> a -> [a]
replicate' n x
  | n <= 0 = []
  | otherwise = x : replicate' (n -1) x

-- 無限に再帰
repeat' :: a -> [a]
repeat' x = x : repeat' x

-- クイックソート
quicksort :: (Ord a) => [a] -> [a]
quicksort [] = []
quicksort (x : xs) =
  let smallerOrEqual = [a | a <- xs, a <= x]
      larger = [a | a <- xs, a > x]
   in quicksort smallerOrEqual ++ [x] ++ quicksort larger

-- セクション
divideByTen :: (Floating a) => a -> a
divideByTen = (/ 10)

-- 高階関数
applyTwice :: (a -> a) -> a -> a
applyTwice f x = f (f x)

-- zipWith
zipWith' :: (a -> b -> c) -> [a] -> [b] -> [c]
zipWith' _ [] _ = []
zipWith' _ _ [] = []
zipWith' f (x : xs) (y : ys) = f x y : zipWith' f xs ys

-- flip
flip' :: (a -> b -> c) -> (b -> a -> c)
flip' f = g
  where
    g x y = f y x

-- map
map' :: (a -> b) -> [a] -> [b]
map' _ [] = []
map' f (x : xs) = f x : map' f xs

-- filter
filter' :: (a -> Bool) -> [a] -> [a]
filter' _ [] = []
filter' p (x : xs)
  | p x = x : filter' p xs
  | otherwise = filter' p xs

-- 10万以下の数のうち3829で割り切れる最大の数
largestDivisible :: Integer
largestDivisible = head (filter p [100000, 99999 ..])
  where
    p x = mod x 3829 == 0

-- 畳み込み
sum' :: (Num a) => [a] -> a
sum' xs = foldl (\acc x -> acc + x) 0 xs

sum'2 :: (Num a) => [a] -> a
sum'2 = foldl (+) 0

-- 関数合成
gousei = map (\x -> negate (abs x)) [5, -3, -6.7, -19, 24]

gousei' = map (negate . abs) [5, -3, -6.7, -19, 24]

gousei2 = map (negate . sum . tail) [[1 .. 5], [3 .. 6], [1 .. 7]]

freepoint = ceiling . negate . tan . cos . max 50

oddSquareSum :: Integer
oddSquareSum = sum . takeWhile (< 10000) . filter odd $ map (^ 2) [1 ..]

main = print  oddSquareSum
