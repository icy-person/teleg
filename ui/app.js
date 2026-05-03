async function load(){

const res=await fetch("../archive/meta.json");

const data=await res.json();

const list=document.getElementById("list");

data.reverse().forEach(p=>{

const div=document.createElement("div");

div.className="item";

div.innerHTML=`
<a href="../${p.file}">${p.title}</a>
<br>
${p.date}
`;

list.appendChild(div);

});

}

load();

document.getElementById("search").oninput=e=>{

const q=e.target.value.toLowerCase();

document.querySelectorAll(".item").forEach(el=>{

el.style.display=el.innerText.toLowerCase().includes(q)
?"block":"none";

});

};
