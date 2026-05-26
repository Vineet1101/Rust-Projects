'use client'

import { useState } from "react"
export default function PriceDisplay({info,companyName,exchange,marketCap,ratio}:{info:{price:number,change:number},companyName:string,exchange:string,marketCap:number,ratio:number}){
    
    const [view,changeView]=useState()
    return(
        <div className="card ">
            <p>Comapny: {companyName}</p>
            <p>Price: {info.price}</p>
            <p>Change: {info.change}</p>
        </div>
    )
}