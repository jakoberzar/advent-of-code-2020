// Learn more about F# at http://fsharp.org
namespace AdventOfCode

module Day5 =
    let inputPath = "./../inputs/day-05.txt"

    exception ParsingException of string

    let mapSeat acc =
        function
        | 'F'
        | 'L' -> acc * 2
        | 'B'
        | 'R' -> acc * 2 + 1
        | _ -> raise (ParsingException("Invalid row/seat character"))

    let getId (line: string): int = line.Trim() |> Seq.fold mapSeat 0

    [<EntryPoint>]
    let main argv =
        let ids =
            inputPath
            |> System.IO.File.ReadAllLines
            |> Seq.map getId

        // Star 1
        let max = Seq.max ids
        printfn "Max seat id is %i" max

        // Star 2
        let sortedIds = Seq.sort ids

        let mySeat: int =
            Seq.zip sortedIds (Seq.skip 1 sortedIds)
            |> Seq.find (fun (left, right) -> left + 1 < right)
            |> fun (left, _) -> left + 1

        printfn "My seat is %i" mySeat

        0 // return an integer exit code
