import React, {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api";
import BingPic from "./bing-pic";
import {Swiper, SwiperSlide} from 'swiper/react';
import 'swiper/css';
import 'swiper/css/navigation';
import 'swiper/css/pagination';
import 'swiper/css/scrollbar';
import {A11y, Grid, Navigation, Pagination, Scrollbar} from "swiper";
import {Alert, Box, Button, CircularProgress, LinearProgress, Snackbar} from "@mui/material";


interface ImageInfo {
    url:string,
    title:string,
    startdate:string
}

interface ResultApi {
    code:string,
    msg:string,
    data:string
}

const bingDomain = "https://cn.bing.com/";

function BingSwiper() {
    const [imageArr,setImageArr] = useState<ImageInfo[]>();
    const [imageInfo,setImageInfo] = useState({url:"",title:"",startdate:""});
    const [success,setSuccess] = useState(false);
    const [error,setError] = useState(false);
    const [msg,setMsg] = useState("");
    const [btnStatus,setBtnStatus] = useState(false);
    const vertical = "top";
    const horizontal = "right";

    useEffect(() => {
        invoke<ImageInfo[]>("get_bing_list").then((info:ImageInfo[]) =>{
            setImageArr(info)
            let image = info.at(0);
            if (image) {
                setImageInfo(image)
            }
        })
    },[]);

    function handlerWallpaper() {
        setBtnStatus(true)
        invoke<ResultApi>("set_wallpaper",{"url":imageInfo.url,"title":imageInfo.title,"date":imageInfo.startdate}).then((rst)=>{
            setMsg(rst.msg)
            if ("success" == rst.data){
                setSuccess(true)
            }else {
                setError(true)
            }
        })
        setBtnStatus(false)

    }

    const handleSuccessClose = (event?: React.SyntheticEvent | Event, reason?: string) => {
        if (reason === 'clickaway') {
            return;
        }
        setSuccess(false);
    };

    const handleErrorClose = (event?: React.SyntheticEvent | Event, reason?: string) => {
        if (reason === 'clickaway') {
            return;
        }
        setError(false)
    };


    return(
        <div>
            <Button disabled={btnStatus} onClick={handlerWallpaper} className={"bing-pic-btn"} size={"small"}  variant="outlined">设置壁纸</Button>

            <Swiper
                modules={[Navigation, Pagination, Scrollbar, A11y,Grid]}
                spaceBetween={0}
                slidesPerView={1}
                pagination={{ clickable: true }}
                onSlideChange={(swiper)=>{
                    if (imageArr){
                        setImageInfo(imageArr[swiper.activeIndex])
                    }
                }}
            >
                {
                    imageArr?.map((image)=>
                        <SwiperSlide key={image.url}>
                            <BingPic url={bingDomain+image.url} title={image.title} key={image.url}/>
                        </SwiperSlide>
                    )
                }
            </Swiper>

            <Snackbar
                anchorOrigin={{ vertical, horizontal }}
                autoHideDuration={2000}
                open={success}
                key={'success'}
                onClose={handleSuccessClose}
            >
                 <Alert sx={{ width: '100%' }} severity="success">{msg}</Alert>
            </Snackbar>
            <Snackbar
                anchorOrigin={{ vertical, horizontal }}
                autoHideDuration={2000}
                open={error}
                key={'error'}
                onClose={handleErrorClose}
            >
                <Alert sx={{ width: '100%' }} severity="error">{msg}</Alert>
            </Snackbar>
        </div>
    )

}

export default BingSwiper