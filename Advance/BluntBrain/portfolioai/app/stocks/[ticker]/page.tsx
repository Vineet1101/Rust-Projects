import PriceDisplay from "./price-display"
import axios from "axios"

export default async function StockPrices({params}:{params:{ticker:string}}){
    const {ticker}=await params
    const response=await fetch(`https://finnhub.io/api/v1/quote?symbol=${ticker}&token=${process.env.FINNHUB_API_KEY}`,{next:{revalidate:60}})
    const quote=await response.json()
    const price={price:quote.c,change:quote.d}
    return (
        <div>
            <h1>{ticker}</h1>
            <PriceDisplay params={price}/>
        </div>
    )
}
