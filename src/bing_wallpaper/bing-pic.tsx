import '../App.css'


function BingPic(props: { url: string | undefined; title: string | undefined; }) {
    return(
       <div className="bing-pic-box">
           <img className="bing-pic" src={props.url} alt={props.title}/>
           <p className="bing-pic-title">{props.title}</p>
       </div>
    )
}

export default BingPic