(function(t){function e(e){for(var n,i,o=e[0],c=e[1],u=e[2],l=0,f=[];l<o.length;l++)i=o[l],Object.prototype.hasOwnProperty.call(a,i)&&a[i]&&f.push(a[i][0]),a[i]=0;for(n in c)Object.prototype.hasOwnProperty.call(c,n)&&(t[n]=c[n]);p&&p(e);while(f.length)f.shift()();return s.push.apply(s,u||[]),r()}function r(){for(var t,e=0;e<s.length;e++){for(var r=s[e],n=!0,i=1;i<r.length;i++){var o=r[i];0!==a[o]&&(n=!1)}n&&(s.splice(e--,1),t=u(u.s=r[0]))}return t}var n={},a={index:0},s=[];function i(t){return u.p+"js/"+({}[t]||t)+"."+{"chunk-d1b2a3de":"d08b3eaa"}[t]+".js"}var o={};var c={a997:function(){return{"./mp4_inspector_bg.js":{__wbg_log_c600c3f1f95af900:function(t,e){return n["1e44"].exports["a"](t,e)},__wbindgen_json_parse:function(t,e){return n["1e44"].exports["b"](t,e)},__wbindgen_throw:function(t,e){return n["1e44"].exports["c"](t,e)}}}}};function u(e){if(n[e])return n[e].exports;var r=n[e]={i:e,l:!1,exports:{}};return t[e].call(r.exports,r,r.exports,u),r.l=!0,r.exports}u.e=function(t){var e=[],r=a[t];if(0!==r)if(r)e.push(r[2]);else{var n=new Promise((function(e,n){r=a[t]=[e,n]}));e.push(r[2]=n);var s,l=document.createElement("script");l.charset="utf-8",l.timeout=120,u.nc&&l.setAttribute("nonce",u.nc),l.src=i(t);var f=new Error;s=function(e){l.onerror=l.onload=null,clearTimeout(d);var r=a[t];if(0!==r){if(r){var n=e&&("load"===e.type?"missing":e.type),s=e&&e.target&&e.target.src;f.message="Loading chunk "+t+" failed.\n("+n+": "+s+")",f.name="ChunkLoadError",f.type=n,f.request=s,r[1](f)}a[t]=void 0}};var d=setTimeout((function(){s({type:"timeout",target:l})}),12e4);l.onerror=l.onload=s,document.head.appendChild(l)}var p={"chunk-d1b2a3de":["a997"]}[t]||[];return p.forEach((function(t){var r=o[t];if(r)e.push(r);else{var n,a=c[t](),s=fetch(u.p+""+{a997:"cf38c462738e6c75fb71"}[t]+".module.wasm");if(a instanceof Promise&&"function"===typeof WebAssembly.compileStreaming)n=Promise.all([WebAssembly.compileStreaming(s),a]).then((function(t){return WebAssembly.instantiate(t[0],t[1])}));else if("function"===typeof WebAssembly.instantiateStreaming)n=WebAssembly.instantiateStreaming(s,a);else{var i=s.then((function(t){return t.arrayBuffer()}));n=i.then((function(t){return WebAssembly.instantiate(t,a)}))}e.push(o[t]=n.then((function(e){return u.w[t]=(e.instance||e).exports})))}})),Promise.all(e)},u.m=t,u.c=n,u.d=function(t,e,r){u.o(t,e)||Object.defineProperty(t,e,{enumerable:!0,get:r})},u.r=function(t){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(t,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(t,"__esModule",{value:!0})},u.t=function(t,e){if(1&e&&(t=u(t)),8&e)return t;if(4&e&&"object"===typeof t&&t&&t.__esModule)return t;var r=Object.create(null);if(u.r(r),Object.defineProperty(r,"default",{enumerable:!0,value:t}),2&e&&"string"!=typeof t)for(var n in t)u.d(r,n,function(e){return t[e]}.bind(null,n));return r},u.n=function(t){var e=t&&t.__esModule?function(){return t["default"]}:function(){return t};return u.d(e,"a",e),e},u.o=function(t,e){return Object.prototype.hasOwnProperty.call(t,e)},u.p="/mp4-inspector/",u.oe=function(t){throw console.error(t),t},u.w={};var l=window["webpackJsonp"]=window["webpackJsonp"]||[],f=l.push.bind(l);l.push=e,l=l.slice();for(var d=0;d<l.length;d++)e(l[d]);var p=f;s.push([0,"chunk-vendors"]),r()})({0:function(t,e,r){t.exports=r("56d7")},"034f":function(t,e,r){"use strict";var n=r("85ec"),a=r.n(n);a.a},"2a73":function(t,e,r){},"4c8d":function(t,e,r){},"56d7":function(t,e,r){"use strict";r.r(e);r("d3b7"),r("e260"),r("e6cf"),r("cca6"),r("a79d");var n=r("2b0e"),a=r("5f5b"),s=function(){var t=this,e=t.$createElement,r=t._self._c||e;return r("div",[r("b-navbar",{attrs:{type:"dark",variant:"dark"}},[r("b-navbar-nav",[r("b-nav-item",{attrs:{href:"#"}},[t._v(" MP4 Inspector ")])],1)],1),r("GitHubCorner"),r("div",{staticClass:"container",attrs:{id:"app"}},[r("div",{staticClass:"mt-2"},[r("b-alert",{attrs:{variant:"warning",show:""}},[t._v("⚠️ Currently a work in progress. Check back for updates.")])],1),r("Inspector")],1),r("footer",{staticClass:"container mt-4 text-center"},[r("hr"),r("div",{staticClass:"text-muted"},[r("ul",[r("li",[t._v(t._s(t.name)+"-"+t._s(t.version))]),t._m(0)])])])],1)},i=[function(){var t=this,e=t.$createElement,r=t._self._c||e;return r("li",[r("a",{attrs:{href:"https://github.com/alfg/mp4-inspector"}},[t._v("GitHub")])])}],o=r("9224"),c=function(){var t=this,e=t.$createElement,r=t._self._c||e;return r("a",{staticClass:"github-corner",attrs:{href:"https://github.com/alfg/mp4-inspector","aria-label":"View source on GitHub"}},[r("svg",{staticStyle:{fill:"rgb(56, 141, 61)",color:"#fff",position:"absolute",top:"0",border:"0",right:"0"},attrs:{width:"80",height:"80",viewBox:"0 0 250 250","aria-hidden":"true"}},[r("path",{attrs:{d:"M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"}}),r("path",{staticClass:"octo-arm",staticStyle:{"transform-origin":"130px 106px"},attrs:{d:"M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,\n      78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,\n      87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2",fill:"currentColor"}}),r("path",{staticClass:"octo-body",attrs:{d:"M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,\n      99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,\n      51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,\n      56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,\n      80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,\n      107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,\n      120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z",fill:"currentColor"}})])])},u=[],l={name:"GitHubCorner"},f=l,d=(r("b134"),r("2877")),p=Object(d["a"])(f,c,u,!1,null,null,null),b=p.exports,h=function(){var t=this,e=t.$createElement,r=t._self._c||e;return r("div",{staticClass:"inspector"},[r("b-form-group",{attrs:{label:"Select a file:","label-for":"file"}},[r("b-form-file",{attrs:{id:"file",accept:"video/mp4",state:Boolean(t.file),placeholder:"Choose a file or drop it here...","drop-placeholder":"Drop file here..."},on:{change:t.onFile},model:{value:t.file,callback:function(e){t.file=e},expression:"file"}})],1),t.showProgress?r("b-progress",{attrs:{height:"2px",value:t.progress,max:"100"}}):t._e(),t.data?r("div",[r("div",{staticClass:"mt-3"},[t._v("Selected file: "+t._s(t.file?t.file.name+": "+t.file.size+" bytes":""))]),r("b-tabs",{staticClass:"mt-4"},[r("b-tab",{staticClass:"mt-2",attrs:{title:"Overview"}},[t.data?r("div",[r("Overview",{attrs:{tracks:t.tracks,info:t.info}})],1):t._e()]),r("b-tab",{staticClass:"mt-2",attrs:{title:"Box Tree"}},[t.data?r("div",{staticClass:"tree-view"},[r("BoxTree",{attrs:{data:t.boxes}})],1):t._e()])],1),r("hr")],1):t._e()],1)},m=[],v=(r("4160"),r("b0c0"),r("5cc6"),r("9a8c"),r("a975"),r("735e"),r("c1ac"),r("d139"),r("3a7b"),r("d5d6"),r("82f8"),r("e91f"),r("60bd"),r("5f96"),r("3280"),r("3fcc"),r("ca91"),r("25a1"),r("cd26"),r("3c5d"),r("2954"),r("649e"),r("219c"),r("170b"),r("b39a"),r("72f7"),r("159b"),r("ade3")),g=function(){var t=this,e=t.$createElement,r=t._self._c||e;return r("div",[r("h4",[t._v("Info")]),r("b-table",{attrs:{stacked:"",items:t.items}}),r("h4",[t._v("Tracks")]),r("b-table",{attrs:{striped:"",hover:"",items:t.tracks}})],1)},_=[],y={name:"Overview",props:["tracks","info"],data:function(){return{items:[{size:this.info.size,major_brand:this.info.major_brand,minor_version:this.info.minor_version,compatible_brands:this.info.compatible_brands,timescale:this.info.timescale,duration:this.info.duration,fragmented:this.info.fragmented}]}}},C=y,w=Object(d["a"])(C,g,_,!1,null,null,null),x=w.exports,j=function(){var t=this,e=t.$createElement,r=t._self._c||e;return r("div",{staticClass:"box-tree"},[r("ul",t._l(t.parsedData,(function(e,n){return r("li",{key:n},[r("span",{staticClass:"key"},[t._v(t._s(e.name.replace("value","")))]),r("code",{staticClass:"value"},[t._v(t._s(t.parsedValue(e.data)))]),[r("BoxTree",{attrs:{data:e.data}})]],2)})),0)])},k=[],O=(r("b64b"),r("53ca")),P={name:"BoxTree",props:["data"],computed:{parsedData:function(){var t=this,e=[];return Array.isArray(this.data)?this.data.forEach((function(t){Object.keys(t).forEach((function(r){e.push({name:r,data:t[r]})}))})):"object"===Object(O["a"])(this.data)&&Object.keys(this.data).forEach((function(r){e.push({name:r,data:t.data[r]})})),e}},methods:{parsedValue:function(t){if("number"===typeof t||"string"===typeof t)return t}}},S=P,E=(r("9cfa"),Object(d["a"])(S,j,k,!1,null,"95288138",null)),T=E.exports,A={name:"Inspector",components:{Overview:x,BoxTree:T},data:function(){return{file:null,data:null,progress:0,showProgress:!1}},computed:{info:function(){return this.$mp4.get_media_info(this.data)},tracks:function(){return this.$mp4.get_tracks(this.data)},boxes:function(){var t=this.$mp4.get_boxes(this.data),e=[];return t.forEach((function(t){var r=Object(v["a"])({},t.name,JSON.parse(t.json));e.push(r)})),e}},methods:{onFile:function(t){var e=this;this.data=null,this.progress=0,this.showProgress=!0;var r=t.dataTransfer?t.dataTransfer.files[0]:t.target.files[0],n=new FileReader;n.onload=function(t){e.progress=100,e.data=new Uint8Array(t.target.result),setTimeout((function(){e.showProgress=!1}),2e3)},n.onprogress=function(t){t.lengthComputable&&(e.progress=parseInt(t.loaded/t.total*100,10))},n.readAsArrayBuffer(r)}}},$=A,B=(r("c85a"),Object(d["a"])($,h,m,!1,null,"020e69d4",null)),L=B.exports,M={name:"App",components:{Inspector:L,GitHubCorner:b},data:function(){return{name:o["a"],version:o["b"]}}},I=M,W=(r("034f"),Object(d["a"])(I,s,i,!1,null,null,null)),G=W.exports;r("f9e3"),r("2dd8");r.e("chunk-d1b2a3de").then(r.bind(null,"053d")).then((function(t){n["default"].prototype.$mp4=t,n["default"].use(a["a"]),n["default"].config.productionTip=!1,new n["default"]({render:function(t){return t(G)}}).$mount("#app")}))},"85ec":function(t,e,r){},9224:function(t){t.exports=JSON.parse('{"a":"mp4-inspector-viewer","b":"0.2.1"}')},"9cfa":function(t,e,r){"use strict";var n=r("4c8d"),a=r.n(n);a.a},b134:function(t,e,r){"use strict";var n=r("2a73"),a=r.n(n);a.a},b37b:function(t,e,r){},c85a:function(t,e,r){"use strict";var n=r("b37b"),a=r.n(n);a.a}});
//# sourceMappingURL=index.2cacf526.js.map