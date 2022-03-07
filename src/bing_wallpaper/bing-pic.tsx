import '../App.css'


function BingPic(props: { url: string | undefined; title: string | undefined; }) {
    return(
       <div>
           <img className="bing-pic" src={props.url} alt={props.title}/>
       </div>
    )
}

export default BingPic