import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api";
import BingPic from "./bing-pic";
import { Swiper, SwiperSlide } from 'swiper/react';
import 'swiper/css';
import 'swiper/css/navigation';
import 'swiper/css/pagination';
import 'swiper/css/scrollbar';
import {A11y, Navigation, Pagination, Scrollbar} from "swiper";


interface ImageInfo {
    url:string,
    title:string,
    startdate:string
}
const bingDomain = "https://cn.bing.com/";

function BingSwiper() {
    let [imageArr,setImageArr] = useState<ImageInfo[]>();
    useEffect(() => {
        if (!imageArr){
            invoke<ImageInfo[]>("get_bing_list").then((info) =>{
                setImageArr(info)
            })
        }
    },[imageArr]);


    return(
        <div>
            <Swiper
                modules={[Navigation, Pagination, Scrollbar, A11y]}
                spaceBetween={10}
                slidesPerView={1}
                pagination={{ clickable: true }}
                scrollbar={{ draggable: true }}
                onSlideChange={() => console.log('slide change')}
                onSwiper={(swiper) => console.log(swiper)}
            >
                {
                    imageArr?.map((image)=>
                        <div>
                            <SwiperSlide key={image.startdate}>
                                <BingPic url={bingDomain+image.url} title={image.title}/>
                            </SwiperSlide>
                        </div>
                    )
                }
            </Swiper>
            <p>sss</p>
        </div>

    )

}

export default BingSwiper