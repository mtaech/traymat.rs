import './App.css'

function PicTitle(props:{title: string | undefined}) {
    const title = props.title;
    if (title && title !== "Info"){
        return(
            <p className="bing-pic-title">{props.title}</p>
        )
    }else {
        return (<p className="bing-pic-title"/>)
    }
}

function WallpaperPic(props: { url: string | undefined; title: string | undefined; }) {
    return(
       <div className="bing-pic-box">
           <img className="bing-pic" src={props.url} alt={props.title}/>
           <PicTitle title={props.title}/>
       </div>
    )
}

export default WallpaperPic