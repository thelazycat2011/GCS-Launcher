<script>
	import { Star, Moon, Gem, Coins, Angry, Hammer } from 'lucide-svelte';
	import style from './style.module.scss';
	import ProfilePost from '../../components/ProfilePost/profilePost.svelte';
	import library from '../../libs/library.js';
	
	var isProfileLoaded = '';
	
	var profileData = {
		userName: 'Placeholder',
		stats: {
			stars: 0,
			moons: 0,
			diamonds: 0,
			goldCoins: 0,
			userCoins: 0,
			demons: 0,
			creatorPoints: 0
		},
		posts: []
	};
	
	var profileIcons = {
		cube: "https://gdicon.oat.zone/icon.png?type=cube&value=1&color1=0&color2=3",
		ship: "https://gdicon.oat.zone/icon.png?type=ship&value=1&color1=0&color2=3",
		ball: "https://gdicon.oat.zone/icon.png?type=ball&value=1&color1=0&color2=3",
		ufo: "https://gdicon.oat.zone/icon.png?type=ufo&value=1&color1=0&color2=3",
		wave: "https://gdicon.oat.zone/icon.png?type=wave&value=1&color1=0&color2=3",
		robot: "https://gdicon.oat.zone/icon.png?type=robot&value=1&color1=0&color2=3",
		spider: "https://gdicon.oat.zone/icon.png?type=spider&value=1&color1=0&color2=3",
		swing: "https://gdicon.oat.zone/icon.png?type=swing&value=1&color1=0&color2=3",
		jetpack: "https://gdicon.oat.zone/icon.png?type=jetpack&value=1&color1=0&color2=3"
	};
	
	function getIconURL(type, icon, color1, color2, color3, glow) {
		return "https://gdicon.oat.zone/icon.png?type=" + type + "&value=" + icon + "&color1=" + color1 + "&color2=" + color2 + (glow ? "&glow=1&color3=" + color3 : "");
	}
	
	library.getProfile(localStorage.accountID).then(r => {
		if(r.success) {
			profileData = r.profile;
			if(profileData.icons.currentIcon.iconID == 0) profileData.icons.currentIcon.iconID = 1;
			if(profileData.icons.cube == 0) profileData.icons.cube = 1;
			if(profileData.icons.ship == 0) profileData.icons.ship = 1;
			if(profileData.icons.ball == 0) profileData.icons.ball = 1;
			if(profileData.icons.ufo == 0) profileData.icons.ufo = 1;
			if(profileData.icons.wave == 0) profileData.icons.wave = 1;
			if(profileData.icons.robot == 0) profileData.icons.robot = 1;
			if(profileData.icons.spider == 0) profileData.icons.spider = 1;
			if(profileData.icons.swing == 0) profileData.icons.swing = 1;
			if(profileData.icons.jetpack == 0) profileData.icons.jetpack = 1;
			if(!Object.keys(profileData.clan).length) profileData.clan = {
				name: 'Не в клане',
				color: 'c0c0c0'
			};
			profileIcons = {
				cube: getIconURL('cube', profileData.icons.cube, profileData.icons.colors.mainColor, profileData.icons.colors.secondaryColor, profileData.icons.colors.glowColor, profileData.icons.glow),
				ship: getIconURL('ship', profileData.icons.ship, profileData.icons.colors.mainColor, profileData.icons.colors.secondaryColor, profileData.icons.colors.glowColor, profileData.icons.glow),
				ball: getIconURL('ball', profileData.icons.ball, profileData.icons.colors.mainColor, profileData.icons.colors.secondaryColor, profileData.icons.colors.glowColor, profileData.icons.glow),
				ufo: getIconURL('ufo', profileData.icons.ufo, profileData.icons.colors.mainColor, profileData.icons.colors.secondaryColor, profileData.icons.colors.glowColor, profileData.icons.glow),
				wave: getIconURL('wave', profileData.icons.wave, profileData.icons.colors.mainColor, profileData.icons.colors.secondaryColor, profileData.icons.colors.glowColor, profileData.icons.glow),
				robot: getIconURL('robot', profileData.icons.robot, profileData.icons.colors.mainColor, profileData.icons.colors.secondaryColor, profileData.icons.colors.glowColor, profileData.icons.glow),
				spider: getIconURL('spider', profileData.icons.spider, profileData.icons.colors.mainColor, profileData.icons.colors.secondaryColor, profileData.icons.colors.glowColor, profileData.icons.glow),
				swing: getIconURL('swing', profileData.icons.swing, profileData.icons.colors.mainColor, profileData.icons.colors.secondaryColor, profileData.icons.colors.glowColor, profileData.icons.glow),
				jetpack: getIconURL('jetpack', profileData.icons.jetpack, profileData.icons.colors.mainColor, profileData.icons.colors.secondaryColor, profileData.icons.colors.glowColor, profileData.icons.glow),
			}
			isProfileLoaded = style.hideSkeleton;
		}
	});
</script>

<svelte:head>
	<title>Profile</title>
	<meta name="description" content="Profile" />
</svelte:head>

<div class={style.profileBlock}>
	<div class={[style.head, isProfileLoaded].join(' ')}>
		<div class={style.usernameSkeletonDiv}>
			<span class={[style.skeletonSpan, style.usernameSkeletonSpan].join(' ')}></span>
			<div class={style.usernameDiv}>
				<h1>{profileData.userName}</h1>
			</div>
		</div>
		<div class={style.profileStatsDiv}>
			<div class={style.profileStatsSkeleton}>
				<span class={[style.skeletonSpan, style.profileStatsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileStatsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileStatsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileStatsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileStatsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileStatsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileStatsSkeletonSpan].join(' ')}></span>
			</div>
			<div class={style.profileStat}>
				<Star color="#FFFFFF"/> {profileData.stats.stars}
			</div>
			<div class={style.profileStat}>
				<Moon color="#FFFFFF"/> {profileData.stats.moons}
			</div>
			<div class={style.profileStat}>
				<Gem color="#FFFFFF"/> {profileData.stats.diamonds}
			</div>
			<div class={style.profileStat}>
				<Coins color="#fffd6b"/> {profileData.stats.goldCoins}
			</div>
			<div class={style.profileStat}>
				<Coins color="#FFFFFF"/> {profileData.stats.userCoins}
			</div>
			<div class={style.profileStat}>
				<Angry color="#FFFFFF"/> {profileData.stats.demons}
			</div>
			<div class={style.profileStat}>
				<Hammer color="#FFFFFF"/> {profileData.stats.creatorPoints}
			</div>
		</div>
		<div class={style.profileStatsDiv}>
			<div class={style.profileStatsSkeleton}>
				<span class={[style.skeletonSpan, style.profileIconsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileIconsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileIconsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileIconsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileIconsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileIconsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileIconsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileIconsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileIconsSkeletonSpan].join(' ')}></span>
			</div>
			<div class={style.iconKitIcon}>
				<img class={style.iconBig} src={profileIcons.cube} />
			</div>
			<div class={style.iconKitIcon}>
				<img class={style.iconSmall} src={profileIcons.ship} />
			</div>
			<div class={style.iconKitIcon}>
				<img class={style.iconBig} src={profileIcons.ball} />
			</div>
			<div class={style.iconKitIcon}>
				<img class={style.iconBig} src={profileIcons.ufo} />
			</div>
			<div class={style.iconKitIcon}>
				<img class={style.iconSmall} src={profileIcons.wave} />
			</div>
			<div class={style.iconKitIcon}>
				<img class={style.iconBig} src={profileIcons.robot} />
			</div>
			<div class={style.iconKitIcon}>
				<img class={style.iconBig} src={profileIcons.spider} />
			</div>
			<div class={style.iconKitIcon}>
				<img class={style.iconBig} src={profileIcons.swing} />
			</div>
			<div class={style.iconKitIcon}>
				<img class={style.iconBig} src={profileIcons.jetpack} />
			</div>
		</div>
		<div class={[style.profileStatsDiv, style.profilePostsDiv].join(" ")}>
			<div class={style.profilePostsSkeleton}>
				<span class={[style.skeletonSpan, style.profileIconsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileIconsSkeletonSpan].join(' ')}></span>
				<span class={[style.skeletonSpan, style.profileIconsSkeletonSpan].join(' ')}></span>
			</div>
			{#each profileData.posts as post}
				<ProfilePost username={profileData.userName} postText={post.post} likes={post.likes} dislikes={post.dislikes} timestamp={post.timestamp} />
			{/each}
		</div>
	</div>
</div>