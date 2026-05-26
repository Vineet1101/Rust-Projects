import { Metadata } from "next"
import PriceDisplay from "./price-display"
import { Suspense } from "react";
import { notFound } from "next/navigation";


export async function generateMetadata({params}:{params:{ticker:string}}):Promise<Metadata>{
    const {ticker}=await params;
    return{
        title:`${ticker} Analysis | PortfolioAI`,
        description:`AI-powered bull/bear analysis for ${ticker}`
    }
}


async function getQuote({ticker}:{ticker:string}){
    const response=await fetch(`https://finnhub.io/api/v1/quote?symbol=${ticker}&token=${process.env.FINNHUB_API_KEY}`)
    return response.json()
}

async function getCompanyProfile({ticker}:{ticker:string}){
    const response=await fetch(`https://finnhub.io/api/v1/stock/profile2?symbol=${ticker}&token=${process.env.FINNHUB_API_KEY}`)
    return response.json()
}

async function getBasicFinancials({ticker}:{ticker:string}){
    const response=await fetch(`https://finnhub.io/api/v1/stock/metric?symbol=${ticker}&metric=all&token=${process.env.FINNHUB_API_KEY}`)
    // console.log(response.json())
    return response.json()
}

async function getAIAnalysis(){
    const start=Date.now()
    while(Date.now()-start<1000){
    }
    return (<div>Hey there</div>)
}

export default async function StockPrices({params}:{params:{ticker:string}}){
    const {ticker}=await params

    const [quote,profile,financials]=await Promise.all([getQuote({ticker}),getCompanyProfile({ticker}),getBasicFinancials({ticker})])
    if (!quote.c)notFound()
    const result={info:{price:quote.c,change:quote.d},
        companyName:profile.name,exchange:profile.exchange,marketCap:profile.markteCapitalization,ratio:financials.series.annual.currentRatio[0].v}
    getAIAnalysis()
    return (
        <div>
            <h1>{ticker}</h1>
            <PriceDisplay {...result}/>
            <Suspense fallback={<div>Loading...</div>}>{getAIAnalysis()}</Suspense>
        </div>
    )
}