// Learn more about F# at http://fsharp.org
namespace AdventOfCode

module Day15 =
    open System.Collections.Generic

    let inputPath = "./../inputs/day-15.txt"

    let parseInput (path: string) =
        let text = inputPath |> System.IO.File.ReadAllText
        text.Split "," |> Seq.map int |> Seq.toList

    // Uses an immutable map, which is nicer and more functional but also much slower.
    let playGame (startingNumbers: int list) (turns: int): int =
        let initialMap =
            List.indexed startingNumbers
            |> List.fold (fun acc (idx, number) -> Map.add number (idx + 1) acc) Map.empty

        let (_, last) =
            seq { startingNumbers.Length + 1 .. turns }
            |> Seq.fold (fun (map: Map<int, int>, last: int) turn ->
                let next =
                    match map.TryFind last with
                    | Some (x) -> (turn - 1) - x
                    | None -> 0

                (Map.add last (turn - 1) map, next)) (initialMap, List.last startingNumbers)

        last

    // Uses a mutable dictionary and is much faster, only 5s,
    // compared to 110s using an immutable map
    let playGameDict (startingNumbers: int list) (turns: int): int =
        let dict: Dictionary<int, int> = Dictionary()

        List.indexed startingNumbers
        |> List.iter (fun (idx, number) -> dict.Add(number, (idx + 1)))

        let last =
            seq { startingNumbers.Length + 1 .. turns }
            |> Seq.fold (fun (last: int) turn ->
                let next =
                    if dict.ContainsKey last then
                        let value = (turn - 1) - dict.[last]
                        dict.[last] <- turn - 1
                        value
                    else
                        dict.Add(last, turn - 1)
                        0

                next) (List.last startingNumbers)

        last

    // The solution using arrays takes 18 seconds.
    let playGameArray (startingNumbers: int list) (turns: int): int =
        let arr: int option [] = Array.init turns (fun _ -> None)

        List.indexed startingNumbers
        |> List.iter (fun (idx, number) -> arr.[number] <- Some(idx + 1))

        let last =
            seq { startingNumbers.Length + 1 .. turns }
            |> Seq.fold (fun (last: int) turn ->
                let next =
                    match arr.[last] with
                    | Some (x) -> (turn - 1) - x
                    | None -> 0

                arr.[last] <- Some(turn - 1)

                next) (List.last startingNumbers)

        last

    [<EntryPoint>]
    let main argv =
        let startingNumbers = parseInput inputPath

        let star1 = playGameDict startingNumbers 2020
        printfn "2020th number is %i" star1

        let star2 = playGameDict startingNumbers 30000000
        printfn "30000000th number is %i" star2

        0 // return an integer exit code
